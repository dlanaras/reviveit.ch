import config from './config.json';

/** @type {import('@sveltejs/kit').HandleFetch} */
export async function handleFetch({event, request, fetch}) {

    if (request.url.startsWith(config.apiUrl))
    {
        request.headers.set('cookie', 'auth=' + event.cookies.get('auth'));
    }
    return await fetch(request, {credentials: "include"});
}
