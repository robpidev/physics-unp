import { host } from '$lib/config';
import { error } from '@sveltejs/kit';

export async function load({ cookies }) {


  const url = host + "/course/student/enrolled";
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  console.log(response.status)

  if (response.status === 200) {
    const data = await response.json();
    return {
      courses: data
    };
  }


  if (response.status == 204) {
    return {
      course: null
    };

  }

  if (response.status === 401) {
    throw error(401, 'Unauthorized');
  }

  if (response.status === 400) {
    throw error(400, 'Authorization header not found');
  }

  if (response.status === 404) {
    throw error(404, 'Api Not found');
  }

  throw error(500, 'Internal error server')

}

export const actions = {
  courses: async ({ cookies }) => {
    const url = host + "/course"
    const options = {
      method: 'GET',
      headers: {
        Authorization: cookies.get('token')
      }
    };

    const response = await fetch(url, options);

    if (response.status === 200) {
      const data = await response.json();
      return {
        courses: data
      };
    }

    if (response.status === 400) {
      throw error(400, 'Authorization header not found');
    }

    if (response.status === 401) {
      throw error(401, 'Token invalid');
    }

    throw error(500, 'Internal error server')

  },

  enroll: async ({ request, cookies }) => {
    const data = await request.formData();
    const url = host + '/course/register/' + data.get('course_id');
    const options = {
      method: 'POST',
      headers: {
        Authorization: cookies.get('token')
      }
    };

    try {
      const response = await fetch(url, options);
      const data = await response.text();
      return {
        msj: data,
        ok: true,
      }
    } catch (error) {
      throw error(500, 'Internal error server')
    }
  }
}

