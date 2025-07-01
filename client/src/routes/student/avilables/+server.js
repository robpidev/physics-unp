import { error, json } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function GET({ cookies }) {
  const url = host + "/course/student/avilables"
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  if (response.status === 200) {
    const data = await response.json();
    return json(data)
  }

  throw error(500, 'Internal Sever Error')
}
