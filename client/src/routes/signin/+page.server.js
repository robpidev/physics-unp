import { fail } from "@sveltejs/kit";
import { host } from "$lib/config";

export function load({ cookies }) {
  cookies.delete('token', { path: '/' });
}

export const actions = {
  signin: async ({ request, cookies }) => {
    const data = await request.formData();

    const url = host + '/auth/signin';
    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({ id: data.get('code'), password: data.get('password') })
    };

    const response = await fetch(url, options);

    if (response.status === 400) {
      return fail(400, {
        error: "Código no valido"
      })
    }

    if (response.status === 401) {
      return fail(401, {
        error: "Código o contraseña incorrecta"
      })
    }

    if (response.ok) {
      let data = await response.json();
      cookies.set('token', data.token, { path: '/' });
      return { user: data.user };
    }

    return fail(500, {
      error: "Error de servidor"
    })

  }
}
