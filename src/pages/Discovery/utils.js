export function getFavIconByUrl(url) {
  const matches = url.match(/(https?:\/\/[^\/]*)/i);
  if (matches && matches[1]) {
    return matches[1] + '/favicon.ico';
  } else {
    return null;
  }
}