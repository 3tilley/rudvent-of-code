import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    return {
        todos: await fetch("http://localhost:8000/solution/1").then(
            (data) => data.json()
        )
    }
}