import config from "../../config.json"; 

export const actions = {
	login: async ({ request, cookies }) => {
		const data = await request.formData();
        const body = {
            username: data.get('username')?.toString(),
            password: data.get('password')?.toString()
        }
		await fetch(`${config.apiUrl}/auth`, {method: 'POST', body: JSON.stringify(body)}).then(res => {
            const authCookie = res.headers.getSetCookie()[0];
            cookies.set('auth', getCookieValue('auth', authCookie), {
                httpOnly: true,
                secure: true,
                sameSite: "none",
                expires: getCookieValue('expires', authCookie),
                path: getCookieValue('path', authCookie),
            })
        })
	},
    register: async ({ request }) => {
		const data = await request.formData();
        const body = {
            username: data.get('username')?.toString(),
            password: data.get('password')?.toString(),
            create_key: data.get('register_secret')?.toString()
        }
		await fetch(`${config.apiUrl}/create_admin`, {method: 'POST', body: JSON.stringify(body)})
	}
};
//@ts-ignore
function getCookieValue(name, cookie) 
{
    const regex = new RegExp(`(^| )${name}=([^;]+)`)
    const match = cookie.match(regex)
    if (match) {
        return match[2]
    }
}