import { error, json } from "@sveltejs/kit";

export async function GET({ cookies }) {
  const url = 'http://localhost:8080/professor';
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

