import { request } from "@playwright/test";
import { fail, error } from "@sveltejs/kit";

export async function load({ params, cookies }) {
  const url = 'http://localhost:8080/course/professor/courses/' + params.school;
  const options = {
    method: 'GET',
    headers: {
      Authorization: cookies.get('token')
    }
  };

  const response = await fetch(url, options);

  if (response.status === 200) {
    const courses = await response.json()
    return {
      courses: courses
    }
  }

  // const error = response.text()
  throw error(500, "Internal error server");
}

export const actions = {
  add: async ({ request, cookies, params }) => {
    const data = await request.formData();
    const url = 'http://localhost:8080/course/professor/add';
    const options = {
      method: 'POST',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: cookies.get('token')
      },
      body: new URLSearchParams({
        name: data.get("course").trim(),
        places: '100',
        school_id: params.school
      })
    };

    const response = await fetch(url, options);

    if (response.status == 200) {
      const courses = await response.json();
      console.log(courses)
      return {
        courses: courses
      }
    }

    if (response.status == 400) {
      const error = await response.text()
      console.log(error)
      return fail(400, {
        error
      })
    }

    if (response.status === 401) {
      throw error(401, 'Unauthorized');
    }

    throw error(500, "Internal error server")
  },

  delete: async ({ request, cookies }) => {
    const url = 'http://localhost:8080/course/professor/delete/enyy3yfalin7iq2wj0yc';
    const options = {
      method: 'DELETE',
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
        Authorization: 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjp7ImlkIjoiMTIzNDU2NzgiLCJuYW1lcyI6IlJvYmVyIEVzYmwiLCJsYXN0X25hbWUxIjoiVG9ycmVzdCIsImxhc3RfbmFtZTIiOiJUYXJyaWxsbyIsImdlbmRlciI6dHJ1ZSwicm9sZSI6ImFkbWluIn0sImV4cCI6MzB9.kxERAYEHv2jxL-KT2atdQ84Sg6j5TPZzhjww7bmsqWI'
      },
      body: new URLSearchParams({ name: 'Lab. Física III' })
    };

    try {
      const response = await fetch(url, options);
      const data = await response.json();
      console.log(data);
    } catch (error) {
      console.error(error);
    }
  }
}
