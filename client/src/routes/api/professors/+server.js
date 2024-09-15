import { error, json } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function GET({ cookies }) {
  const url = host + '/professor';
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get("token")
    }
  };

  const response = await fetch(url, options);

  if (response.status == 200) {
    const professors = await response.json();
    return json(professors)

  }

  // TODO: Manejar este error
  throw error(500, "Internal error")
}

