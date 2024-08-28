import { error } from "@sveltejs/kit";
export async function load({ params, cookies }) {
  const url = 'http://localhost:8080/course/professor/professors/' + params.course;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get("token")
    }
  };

  const response = await fetch(url, options);

  if (response.status == 200) {
    const professors = await response.json();
    return {
      professors
    }
  }

  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  throw error(500, "Internal error server")

}

export const actions = {
  assign: async ({ request, cookies, params }) => {

    let data = await request.formData()

    const url = 'http://localhost:8080/course/professor/asign';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams({ course_id: params.course, user_id: data.get("user_id"), role: data.get("role") })
    };

    const response = await fetch(url, options);

    if (response.status === 200) {
      return {
        ok: true
      }
    }

    throw error(500, "Internal error server")
  },

  unassign: async ({ request, cookies, params }) => {
    let data = await request.formData()

    const url = 'http://localhost:8080/course/professor/asign';
    const options = {
      method: 'DELETE',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams({ course_id: params.course, user_id: data.get("user_id") })
    };

    const response = await fetch(url, options);
    if (response.status == 200) {
      return { ok: true }
    }

    throw error(500, "Internal error server")
  }
}
