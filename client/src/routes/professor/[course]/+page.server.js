import { error, fail } from "@sveltejs/kit";
import { host } from "$lib/config";


export async function load({ params, cookies }) {

  let url = host + "/evaluation/professor/all/" + params.course;

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
    console.log(await response.text())
    throw error(401, "Authorization no valid")
  }

  if (response.status == 404) {
    throw error(500, "Api resource not found")
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

  if (response.status == 404) {
    throw error(500, response.status + " Api Course not found")
  }

  throw error(500, "Internal error server: " + response.status)

}

export const actions = {
  updateponderation: async ({ request, cookies, params }) => {

    let data = await request.formData()


    const test = Number(data.get("test"))
    const practice = Number(data.get("practice"))

    if (test + practice > 100 || test + practice < 0 || test < 0 || practice < 0) {
      throw error(400, "Invalid ponderation")
    }

    const url = host + "/course/professor/" + params.course;
    const options = {
      method: 'PATCH',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams(
        {
          test: Number(data.get("test")),
          practice: Number(data.get("practice")),
        })
    };

    const response = await fetch(url, options);
    if (response.status == 200) {
      return { tests: await response.json() }
    }

    if (response.status == 401) {
      throw error(401, "Authorization no valid")
    }

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    throw error(500, "Internal error server" + response.status + " " + await response.text())
  },

  update_score: async ({ request, cookies, params }) => {
    let data = await request.formData();
    let url = host + "/evaluation/professor";

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
      const error = await response.text();
      if (error.includes("Update no avilable")) {
        return fail(400, {
          error: "No está abilitada la actualización de esta nota"
        })
      }

      return fail(400, {
        error
      })
    }

    throw error(500, "Internal error server")
  },

  add_score: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = host + "/evaluation/professor";
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

      const error = await response.text();

      if (error.includes("Register no avilable")) {
        return fail(400, {
          error: "El registro para esta nota no está abilitado"
        })
      }

      return fail(400, {
        error
      })
    }

    throw error(500, "Internal error server")
  }
}
