import config from "../../config.json"; 

export const actions = {
	login: async ({ request, cookies, fetch }) => {
		const data = await request.formData();
        const body = {
            username: data.get('username')?.toString(),
            password: data.get('password')?.toString()
        }
		await fetch(`${config.apiUrl}/auth`, {method: 'POST', body: JSON.stringify(body)}).then(res => {
            setAuthCookie(res, cookies);
        })
	},
    register: async ({ request, cookies, fetch }) => {
		const data = await request.formData();
        const body = {
            username: data.get('username')?.toString(),
            password: data.get('password')?.toString(),
            create_key: data.get('register_secret')?.toString()
        }
		await fetch(`${config.apiUrl}/create_admin`, {method: 'POST', body: JSON.stringify(body)}).then(res => {
            setAuthCookie(res, cookies);
        })
	}
};

//@ts-ignore
function setAuthCookie(res, cookies) {
    const authCookie = res.headers.get('set-cookie');
    console.log(authCookie, res.headers);
    cookies.set('auth', getCookieValue('auth', authCookie), {
        httpOnly: true,
        secure: true,
        sameSite: "none",
        expires: new Date(getCookieValue('Expires', authCookie)),
        path: getCookieValue('Path', authCookie)
    });
}

//@ts-ignore
function getCookieValue(name, cookie) 
{
    const regex = new RegExp(`(^| )${name}=([^;]+)`)
    const match = cookie.match(regex)
    if (match) {
        return match[2]
    }
}