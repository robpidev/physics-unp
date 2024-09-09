import { fail, error } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function load({ cookies }) {

  let url = host + '/faculty'
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  let response = await fetch(url, options);

  if (!cookies.get('token')) {
    throw error(401, 'Token invalid');
  }

  if (response.status === 401) {
    throw error(401, 'Unauthorized');
  }

  let faculties;

  if (response.ok) {
    faculties = await response.json();
  }

  url = host + '/calendar';
  response = await fetch(url, options);

  if (response.ok) {
    const calendar = await response.json();
    return {
      faculties,
      calendar
    }
  }

  if (response.status === 401) {
    throw error(401, 'Unauthorized');
  }

  throw error(500, '  Internal error server');

}

export const actions = {
  add: async ({ request, cookies }) => {
    const data = await request.formData();
    if (!data.get('name')) {
      return fail(400, {
        error: 'Name is required'
      })
    }

    const url = host + '/faculty/add';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({ name: data.get('name') })
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

    throw error(500, 'Internal error server')
  }
}
