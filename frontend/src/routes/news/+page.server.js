import config from "../../config.json"; 

/** @type {import('./$types').PageServerLoad} */
export async function load() {
    return { 
        /**@type {import('../../models/Article').Article[]} */
        articles: await fetch(`${config.apiUrl}/articles`).then(response => response.json())
    };
}
