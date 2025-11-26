import { host } from "$lib/config";
import { error } from '@sveltejs/kit';
import { fail } from '@sveltejs/kit';


export async function load() {

  const url = host + "/notices"
  const options = { method: 'GET' };


  const response = await fetch(url, options);


  if (response.status === 404) {
    throw error(404, 'Not found')
  }

  if (response.status === 200) {
    const data = await response.json()
    return {
      notices: data
    }
  }

  throw error(500, 'Internal error server')
}


export const actions = {
  remove: async ({ request, cookies }) => {
    const data = await request.formData();
    const id = data.get('id');


    const url = host + '/notices/admin/' + id;
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

    console.log(response)

    throw error(500, 'Internal error server');
  },

  add: async ({ request, cookies }) => {
    const form = await request.formData();

    const note = form.get('note');

    console.log(note)

    if (!note || note.length === 0) {
      return fail(400, {
        error: 'Note is required'
      })
    }

    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({ note })
    };


    const url = host + '/notices/admin';
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


    console.log(response)
    throw error(500, 'Internal error server');

  }

}
