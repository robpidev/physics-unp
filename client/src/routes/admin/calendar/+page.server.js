import { fail, error } from '@sveltejs/kit';
import { host } from "$lib/config";

export async function load({ cookies }) {

  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const url = host + '/calendar';
  const response = await fetch(url, options);

  if (response.ok) {
    const calendar = await response.json();
    return {
      calendar
    }
  }

  if (response.status === 401) {
    throw error(401, 'Unauthorized');
  }

  throw error(500, 'Internal error server');

}


export const actions = {
  remove: async ({ request, cookies }) => {
    const data = await request.formData();
    const id = data.get('id');


    const url = host + '/calendar/' + id;
    const options = {
      method: 'delete',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
    };


    const response = await fetch(url, options);


    if (response.ok) {
      return {
        success: true
      }
    }

    if (response.status === 400) {
      const data = await response.text();
      return fail(400, {
        error: data
      })
    }

    throw error(500, 'Internal error server');
  },

  add: async ({ request, cookies }) => {
    const form = await request.formData();

    const todo = form.get('todo');
    let hour = form.get('hour');
    let minute = form.get('minute');
    let end = form.get('end');


    if (String(minute).length === 1) {
      minute = "0" + minute
    }


    hour = String((5 + Number(hour)) % 24);
    hour = hour.length === 1 ? "0" + hour : hour;
    end = end + "T" + hour + ":" + minute + ":00Z";


    console.log(todo, end);
    const url = host + '/calendar';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({ todo: todo, end: end })
    };


    const response = await fetch(url, options);

    if (response.ok) {
      return {
        success: true
      }
    }

    if (response.status === 400) {
      const data = await response.text();
      return fail(400, {
        error: data
      })
    }

  }
}
