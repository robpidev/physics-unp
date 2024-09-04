import { host } from "$lib/config";
export async function load({ cookies }) {
  const url = host + '/course/professor/courses';
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  if (!cookies.get('token')) {
    throw error(401, 'Token invalid');
  }

  if (response.status === 401) {
    throw error(401, 'Unauthorized');
  }

  if (response.ok) {
    const data = await response.json();
    return {
      courses: data
    }
  }

  throw error(500, '  Internal error server');

}


