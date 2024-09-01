import { error } from "@sveltejs/kit";

const host = "http://localhost:8080";

export async function load({ params, cookies }) {
  let url = host + '/course/professor/professors/' + params.course;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get("token")
    }
  };

  let response = await fetch(url, options);

  let professors;
  if (response.status == 200) {
    professors = await response.json();
  }

  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  url = host + "/evaluation/all/" + params.course;


  response = await fetch(url, options);
  let evaluations;
  if (response.status == 200) {
    evaluations = await response.json();
  }

  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  url = host + "/course/professor/" + params.course;
  response = await fetch(url, options);
  if (response.status == 200) {
    const course = await response.json();
    return {
      course,
      evaluations,
      professors,
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
  },

  updateponderation: async ({ request, cookies, params }) => {

    let data = await request.formData()

    const url = host + "/course/professor/" + params.course;
    const options = {
      method: 'PATCH',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams(
        {
          test: data.get("test"),
          practice: data.get("practice")
        })
    };

    const response = await fetch(url, options);
    if (response.status == 200) {
      return { tests: await response.json() }
    }

    if (response.status == 401) {
      throw error(401, "Authorization no valid")
    }

    throw error(500, "Internal error server")
  }
}
