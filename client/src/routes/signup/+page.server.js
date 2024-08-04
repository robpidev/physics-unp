import { fail } from "@sveltejs/kit";

export async function load() {
  const url = 'http://localhost:8080/school';
  const options = {
    method: 'GET',
    headers: { 'content-type': 'application/x-www-form-urlencoded' },
    //body: new URLSearchParams({ '': '' })
  };

  try {
    const response = await fetch(url, options);
    let data = await response.json();
    return { schools: data };
  } catch (error) {
    fail(500, {
      error: "Error de servidor"
    })
  }
}

export const actions = {
  signup: async ({ request }) => {
    let data = await request.formData();

    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams(data)
    };

    let url;

    if (data.get("code") !== null) {
      url = 'http://127.0.0.1:8080/auth/signup/student';

    } else if (data.get("dni") !== null) {
      url = 'http://127.0.0.1:8080/auth/signup/professor';
    }
    console.log(data)

    try {
      const response = await fetch(url, options);
      if (response.status === 400) {

        let data = await response.text();
        return fail(400, {
          error: "Datos invalidos",
          message: data
        })
      }

      if (response.status === 500) {
        return fail(500, {
          error: "Error de servidor"
        })
      }

      if (response.ok) {
        let data = await response.json();
        console.log(data)
        return { user: data }
      }

    } catch (error) {
      console.error(error);
      return fail(500, {
        error: "Error de servidor: " + error
      })
    }
  }
}
