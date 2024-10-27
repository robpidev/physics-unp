import { error, fail } from "@sveltejs/kit";
import { host } from "$lib/config";

export async function load({ params, cookies }) {
  let url = host + '/course/admin/' + params.course;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get("token")
    }
  };

  let response = await fetch(url, options);

  let course;
  if (response.status == 200) {
    course = await response.json();
  }


  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  if (response.status == 404) {
    throw error(404, "Course not found")
  }

  url = host + "/evaluation/professor/all/" + params.course;

  response = await fetch(url, options);
  if (response.status == 200) {
    const evaluations = await response.json();
    return {
      course,
      evaluations,
    }
  }

  if (response.status == 401) {
    throw error(401, "Authorization no valid")
  }

  if (response.status == 404) {
    throw error(404, "Api resource not found")
  }

  throw error(500, "Internal error server")

}

//** @satisfies {import('./$types').Actions} */
export const actions = {
  assign: async ({ request, cookies, params }) => {

    let data = await request.formData()

    const url = host + "/course/admin/professor/asign";
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


    const url = host + "/course/admin/professor/asign";
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

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    throw error(500, "Internal error server: " + response.status + " " + await response.text())
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

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    throw error(500, "Internal error server")
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
      throw error(401, "Authorization no valid")
    }

    if (response.status == 404) {
      throw error(404, "Api resource not found")
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
      return fail(400, {
        error: await response.text(),
      })
    }

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    throw error(500, "Internal error server")
  },

  // places
  updatePlaces: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = host + "/course/admin/places";
    const options = {
      method: 'PATCH',
      headers: {
        'content-type': 'application/json',
        Authorization: cookies.get("token")
      },
      body: JSON.stringify({
        places: Number(data.get("places")),
        course_id: params.course
      })
    };



    const response = await fetch(url, options);


    if (response.status == 200) {
      return {
        ok: true
      }
    }


    if (response.status == 400) {
      return fail(400, {
        error: await response.text(),
      })
    }

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    throw error(500, "Internal error server")
  },

  studentInfo: async ({ request, cookies }) => {
    const data = await request.formData();
    const url = host + "/student/" + data.get("student_id");
    const options = {
      method: 'GET',
      headers: {
        'content-type': 'application/json',
        Authorization: cookies.get("token")
      },
    }

    const response = await fetch(url, options);

    if (response.status == 200) {
      const student = await response.json();
      return {
        student
      }
    }


    if (response.status == 400) {
      return fail(400, {
        error: await response.text(),
      })
    }

    if (response.status == 404) {
      return fail(404, {
        error: await response.text(),
      })
    }

    const msj = await response.text();

    if (msj.includes("Expected")) {
      return fail(400, {
        error: "Code no valid"
      })
    }

    throw error(500, "Internal error server: " + response.status + await response.text())
  },

  enroll: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = host + "/course/admin/enroll";
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get("token")
      },
      body: new URLSearchParams({
        student_id: data.get("student_id"),
        course_id: params.course
      })
    }


    const response = await fetch(url, options);

    if (response.status == 200) {
      return {
        ok: true
      }
    }

    if (response.status == 404) {
      throw error(404, "Api resource not found")
    }

    if (response.status == 400) {
      return fail(400, {
        error: await response.text(),
      })
    }

    throw error(500, "Internal error server: " + response.status + await response.text())
  }
}
