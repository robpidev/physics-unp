import { error, fail } from "@sveltejs/kit";
import { host } from "$lib/config";


export async function load({ params, cookies }) {

  let url = host + "/evaluation/all/" + params.course;

  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get("token")
    }
  };

  let response = await fetch(url, options);
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
    }
  }

  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  throw error(500, "Internal error server")

}

export const actions = {
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
  },

  update_score: async ({ request, cookies, params }) => {
    let data = await request.formData();
    let url = host + "/evaluation";

    const options = {
      method: 'PATCH',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams(
        {
          score: data.get("score"),
          ev_id: data.get("ev_id"),
          course_id: params.course,
          number: data.get("number"),
        }
      )
    };

    const response = await fetch(url, options);

    if (response.status == 200) {
      return {
        ok: true
      }
    }

    if (response.status == 401) {
      console.log(await response.text())
      throw error(401, "Authorization no valid")
    }

    if (response.status == 400) {
      return fail(400, {
        error: await response.text(),
      })
    }

    throw error(500, "Internal error server")
  },

  add_score: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = host + "/evaluation";
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams({
        course_id: params.course,
        student_id: data.get("student_id"),
        evaluation_type: data.get("ev_type"),
        score: data.get("score"),
        number: data.get("number"),
      })
    };

    const response = await fetch(url, options);

    if (response.status == 200) {
      return {
        ok: true
      }
    }

    if (response.status == 401) {
      throw error(401, "Authorization no valid")
    }

    if (response.status == 400) {
      return fail(400, {
        error: await response.text(),
      })
    }

    throw error(500, "Internal error server")
  }
}