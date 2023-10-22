const BASE_URL = 'https://wall-paper.online/wp';

export function getImgs() {
  const apiUrl = `${BASE_URL}/imgs`;
  return fetch(apiUrl, {
    method: 'POST',
    mode: 'cors',
    referrerPolicy: "no-referrer"
  }).then(response => {
    if (response.status === 200) {
      return response.json();
    }
  })
}

export function getSites() {
  const apiUrl = `${BASE_URL}/recsites`;
  return fetch(apiUrl, {
    method: 'POST',
    mode: 'cors',
    referrerPolicy: "no-referrer"
  }).then(response => {
    if (response.status === 200) {
      return response.json();
    }
  })
}