import { solve } from '$lib/challange';
import { invalid } from '@sveltejs/kit';
import type { Actions } from './$types';
import { env } from '$env/dynamic/private';

const GITHUB_TOKEN = env.GITHUB_TOKEN;

const trigger = (inputs: any) =>
    fetch(
        'https://api.github.com/repos/ngalaiko/galaiko.rocks/actions/workflows/create-comment.yaml/dispatches',
        {
            method: 'POST',
            headers: {
                Accept: 'application/vnd.github+json',
                Authorization: `Bearer ${GITHUB_TOKEN}`
            },
            body: JSON.stringify({
                ref: 'master',
                inputs
            })
        }
    );

export const actions: Actions = {
    default: async ({ request }) => {
        const data = await request.formData();
        const body = data.get('body');
        const author_name = data.get('author_name');
        const pathname = data.get('pathname');
        const solution = data.get('solution');
        const challange = data.get('challange');

        if (body === '') {
            return invalid(400, { message: 'Message can not be empty' });
        }

        if (author_name === '') {
            return invalid(400, { message: 'Please, fill in name' });
        }

        if (solution === '') {
            return invalid(400, { message: 'Challange solution is empty' });
        } else if (solution !== solve(challange as string)) {
            return invalid(400, { message: 'Wrong solution' });
        } else {
            const res = await trigger({ body, author_name, pathname });
            if (res.status !== 204) {
                return invalid(500, { message: await res.json() });
            } else {
                return {
                    success: true,
                    message: 'Thanks! Your comment will appear after moderation. Check in later!'
                };
            }
        }
    }
};
