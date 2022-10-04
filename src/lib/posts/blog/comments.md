---
title: 'GitHub powered comments for a static website'
tags: ['blog', 'sveltekit']
date: 2022-10-04
categories: ['Blog']
---

after [implementing and then removing indieweb's webmentions][], i still kept thinking about a good way to allow comments on this website.
a good way for me is:

- without a dedicated server software that i have to run and maintain
- works without javascript
- doesn't share data with 3rd parties

and last week [this post][] poped up in my rss feed.
it's written in russian, so i'll summarize it briefly.

the author noticed that disqus (a cloud comments engine) that he uses sends a lot of garbage js requests and decided to fix that.
the fix is neat: use github pull requests to create website comments.

i really like the idea, so now my website has comments that work in a silimar (yet a little beter) way!
here is how:

1. i have a github [action][] to create a new comment.
   it's really simple: given a author name, url and comment body, create a new .md file with the coment body like this:

   ```md
   ---
   pathname: '/posts/blog/cluster/'
   author-name: 'Nikita'
   timestamp: '1664822102'
   ---

   comment body
   ```

2. a [sveltekit form][] to trigger action via http:

   ```svelte
   <form method="POST" use:enhance>
    <!-- author name input -->
    <label for="author_name"> Your name: </label>
    <input name="author_name" type="text" required />

    <!-- challange text -->
    <label for="solution">{data.challange} = </label>
    <input name="solution" type="text" required />

    <!-- comment body -->
    <textarea rows="3" name="body" type="text" required />

    <!-- url pathname to link comment to the page -->
    <input name="pathname" value="{$page.url.pathname}" type="text" hidden />

    <!-- input challange for validation -->
    <input name="challange" value="{data.challange}" type="text" hidden />

    <input type="submit" value="Comment" />
   </form>
   ```

3. some server-side typescript to trigger the action:

   ```ts
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

   		if (body === '') return invalid(400, { message: 'Message can not be empty' });
   		if (author_name === '') return invalid(400, { message: 'Please, fill in name' });

   		if (solution === '') {
   			return invalid(400, { message: 'Challange solution is empty' });
   		} else if (solution !== solve(challange as string)) {
   			return invalid(400, { message: 'Wrong solution' });
   		} else {
   			const res = await trigger({ body, author_name, pathname });
   			if (res.status !== 204) {
   				return invalid(500, { message: await res.text() });
   			} else {
   				return {
   					success: true,
   					message: 'Thanks! Your comment will appear after moderation. Check in later!'
   				};
   			}
   		}
   	}
   };
   ```

and that's it! thanks to sveltekit, the implementation is both nice, js-free and i don't need to think about hosting

[this post]: https://grishaev.me/de-js-3/
[action]: https://github.com/ngalaiko/galaiko.rocks/blob/1d1c6d6858250272814a1f60bd18e74d8018f9e2/.github/workflows/create-comment.yaml
[sveltekit form]: https://kit.svelte.dev/docs/form-actions
[implementing and then removing indieweb's webmentions]: /posts/blog/hello-indieweb/
