import { host } from "$lib/config";
import { error } from '@sveltejs/kit';

export async function load() {

  const url = host + "/notices"
  const options = { method: 'GET' };


  const response = await fetch(url, options);


  if (response.status === 404) {
    throw error(404, 'Not found')
  }

  if (response.status === 200) {
    const data = await response.json()
    // console.log(data)
    return {
      notices: data
    }
  }



  throw error(500, 'Internal error server')
}
