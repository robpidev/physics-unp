import { fail, error } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function load({ params, cookies }) {
  const url = host + '/course/admin/school/' + params.school;
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

    const url = host + '/course/admin';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({
        name: data.get("course").trim(),
        places: Number(data.get("places")),
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
    const data = await request.formData();
    const url = host + '/course/admin/' + data.get('course_id');
    const options = {
      method: 'DELETE',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({ name: 'Lab. Física III' })
    };

    const response = await fetch(url, options);

    if (response.status == 200) {
      return {
        ok: true
      }
    }

    if (response.status == 400) {
      const error = await response.text()
      return fail(400, {
        msg: "Curso no existe"
      })
    }

    if (response.status == 401) {
      throw error(401, 'Unauthorized');
    }

    if (response.status == 404) {
      throw error(404, 'Api resource not found');
    }


    throw error(500, "Internal error server")
  }
}
