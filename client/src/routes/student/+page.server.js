import { error } from '@sveltejs/kit';

export async function load({ cookies }) {
  const url = 'http://localhost:8080/course/enrolled';
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
      course: data
    };
  }


  if (response.status == 204) {
    return {
      course: null
    };

  }

  if (response.status === 401) {
    throw new error(401, 'Unauthorized');
  }

  if (response.status === 400) {
    throw new error(400, 'Authorization header not found');
  }

  throw new error(500, 'Internal error server')

}

export const actions = {
  courses: async ({ cookies }) => {
    const url = 'http://localhost:8080/course';
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
      throw new error(400, 'Authorization header not found');
    }

    if (response.status === 401) {
      throw new error(401, 'Token invalid');
    }

    throw new error(500, 'Internal error server')

  },

  enroll: async ({ request, cookies }) => {
    const data = await request.formData();
    const url = 'http://localhost:8080/course/register/' + data.get('course_id');
    const options = {
      method: 'POST',
      headers: {
        Authorization: cookies.get('token')
      }
    };

    try {
      const response = await fetch(url, options);
      const data = await response.text();
      console.log(data);
    } catch (error) {
      console.error(error);
    }
  }
}

