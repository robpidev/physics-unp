import { fail, error } from "@sveltejs/kit";

export async function load({ params, cookies }) {
  const url = 'http://localhost:8080/course/professor/courses/' + params.school;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  if (response.status === 200) {
    const schools = await response.json()
    console.log(schools)
    return {
      schools
    }
  }

  const error = response.text()
  throw new error(500, "Internal error server")
}

export const actions = {
  add: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = 'http://localhost:8080/course/professor/add';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({
        name: data.get("course").trim(),
        places: '100',
        school_id: params.school
      })
    };

    const response = await fetch(url, options);

    if (response.status == 200) {
      const school = await response.json();
      return {
        school
      }
    }

    if (response.status == 400) {
      const error = await response.text()
      console.log(error)
      return fail(400, {
        error
      })
    }

    if (response.status === 401) {
      throw new error(401, 'Unauthorized');
    }

    throw new error(500, "Internal error server")
  }
}
