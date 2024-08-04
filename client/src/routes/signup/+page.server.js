import { fail } from "@sveltejs/kit";

export async function load() {
  const url = 'http://localhost:8080/school';
  const options = {
    method: 'GET',
    headers: { 'content-type': 'application/x-www-form-urlencoded' },
    //body: new URLSearchParams({ '': '' })
  };

  try {
    const response = await fetch(url, options);
    let data = await response.json();
    return { schools: data };
  } catch (error) {
    fail(500, {
      error: "Error de servidor"
    })
  }
}
