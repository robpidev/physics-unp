import { fail, error } from "@sveltejs/kit";

export async function load({ params }) {
  const url = 'http://localhost:8080/school/' + params.faculty;
  const options = { method: 'GET', headers: { 'Content-Type': '' } };

  const response = await fetch(url, options);

  console.log(params.faculty)

  if (response.status === 200) {
    const data = await response.json()

    console.log(data)
    return {
      data
    }
  }

  const error = response.text()
  throw new error(500, "Internal error server")
}

export const actions = {
  add: async ({ request, cookies, params }) => {
    const data = await request.formData();
    if (!data.get('school')) {
      return fail(400, {
        error: 'Name is required'
      })
    }

    const url = 'http://localhost:8080/school/admin/add';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams({ name: data.get('school'), faculty_id: params.faculty })
    };

    const response = await fetch(url, options);

    if (response.status === 400) {
      const data = await response.text();
      return fail(400, {
        error: data,
      })
    }

    if (response.status === 500) {
      const data = await response.text();
      return fail(500, {
        error: data,
      })
    }

    if (response.ok) {
      return {
        success: true
      }
    }

    throw new error(500, 'Internal error server' + await response.text())
  }
}
