import { host } from "$lib/config";
import { error } from '@sveltejs/kit';
export async function load({ cookies }) {
  const url = host + '/course/professor';
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
    const courses = await response.json();
    return {
      courses
    }
  }

  throw error(500, 'Internal error server');

}
