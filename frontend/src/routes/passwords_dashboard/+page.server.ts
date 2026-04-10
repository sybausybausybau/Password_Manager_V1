import type { PageServerLoad } from './$types';

export const load = (async ({cookies}) => {
    let token = cookies.get("token");
    return { token : token };
}) satisfies PageServerLoad;