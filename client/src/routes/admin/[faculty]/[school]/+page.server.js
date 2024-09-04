import { fail, error } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function load({ params, cookies }) {
  const url = host + '/course/professor/courses/' + params.school;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  if (response.status === 200) {
    const courses = await response.json()
    return {
      courses: courses
    }
  }

  // const error = response.text()
  throw error(500, "Internal error server");
}

export const actions = {
  add: async ({ request, cookies, params }) => {
    const data = await request.formData();

    const url = host + '/course/professor/add';
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
      const courses = await response.json();
      return {
        courses: courses
      }
    }

    if (response.status == 400) {
      const error = await response.text()
      return fail(400, {
        error
      })
    }

    if (response.status === 401) {
      throw error(401, 'Unauthorized');
    }

    throw error(500, "Internal error server")
  },

  delete: async ({ request, cookies }) => {
    const url = host + '/course/professor/delete/enyy3yfalin7iq2wj0yc';
    const options = {
      method: 'DELETE',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({ name: 'Lab. Física III' })
    };

    try {
      const response = await fetch(url, options);
      const data = await response.json();
      console.log(data);
    } catch (error) {
      console.error(error);
    }
  }
}
