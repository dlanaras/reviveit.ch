import config from "../../../config.json"; 

export const actions = {
	default: async ({ request, fetch }) => {
		const data = await request.formData();
        //@ts-ignore
        const dateAsUnixTime = new Date(data.get('date').toString()).getTime() / 1000;
        const body = {
            title: data.get('title')?.toString(),
            content: data.get('content')?.toString(),
            date: dateAsUnixTime
        }
		await fetch(`${config.apiUrl}/articles`, {method: 'POST', body: JSON.stringify(body), credentials: 'include'})
	}
};
