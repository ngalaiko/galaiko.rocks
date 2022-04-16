<script lang="ts">
	import { onMount } from 'svelte';

	let angle = 0;
	const angleIncrement = (0.5 * Math.PI) / 128;
	const TWO_PI = Math.PI * 2;
	const offsetBase = 1.55;

	const rotateOffsets = async (ratio: number, distanceMultiplier: number) => {
		const main = document.getElementById('main');

		angle += angleIncrement * ratio;
		angle %= TWO_PI;
		main.style.setProperty('--moveX', distanceMultiplier * offsetBase * Math.cos(angle) + 'px');
		main.style.setProperty('--moveY', distanceMultiplier * offsetBase * Math.sin(angle) + 'px');
	};

	const update = () => {
		requestAnimationFrame((elapsed) => {
			stepper(elapsed);
			update();
		});
	};

	let lastElapsed = 0;
	const BASE_DELTA = 1000 / 60; // 1s / 60 frames
	let lastDelta = BASE_DELTA;

	const stepper = (elapsed: number) => {
		lastDelta = elapsed - lastElapsed;
		lastElapsed = elapsed;
		const ratio = lastDelta / BASE_DELTA;
		if (Date.now() - lastMove > 200) {
			mouseTravel = Math.max(0, mouseTravel - ratio * 70);
		}
		rotateOffsets(ratio, Math.max(1, Math.min(mouseTravel / 200, 10)));
	};

	const lastMousePos = [-1, -1];
	let mouseTravel = 0;
	let lastMove = 0;
	const mouseMoveListener = ({ pageX, pageY }) => {
		if (lastMousePos[0] > -1)
			mouseTravel += Math.max(Math.abs(pageX - lastMousePos[0]), Math.abs(pageY - lastMousePos[1]));
		lastMousePos[0] = pageX;
		lastMousePos[1] = pageY;
		lastMove = Date.now();
	};

	onMount(() => {
		window.addEventListener('mousemove', mouseMoveListener);
		document.querySelectorAll('main a').forEach((link: HTMLLinkElement) => {
			link.dataset['text'] = link.innerText;
			link.style.setProperty('text-decoration', 'none');
		});
		update();
	});
</script>

<div class="glitchy-links">
	<slot />
</div>

<style>
	:global(.glitchy-links a) {
		display: inline-block;
		color: var(--color-blue);
		position: relative;
		font-weight: 500;
		text-decoration: none;
		transition: color 0.5s linear 0.2s;
	}

	:global(.glitchy-links a:after),
	:global(.glitchy-links a:before) {
		transition: transform 0.5s ease-out, opacity 0.5s linear;
		opacity: 0;
		content: attr(data-text);
		position: absolute;
		top: 0;
		left: 0;
	}

	:global(.glitchy-links a:hover:after),
	:global(.glitchy-links a:hover:before) {
		opacity: 0.75;
	}

	:global(.glitchy-links a:before) {
		color: var(--color-red);
	}

	:global(.glitchy-links a:after) {
		color: var(--color-blue);
	}

	:global(.glitchy-links a:focus),
	:global(.glitchy-links a:hover) {
		color: var(--color-green);
		transition-delay: 0s;
	}

	:global(.glitchy-links a:focus:before),
	:global(.glitchy-links a:hover:before) {
		transform: translate3d(var(--moveOffset), var(--moveOffset), 0);
		transform: translate3d(var(--moveX, var(--moveOffset)), var(--moveY, var(--moveOffset)), 0);
	}

	:global(.glitchy-links a:focus:after),
	:global(.glitchy-links a:hover:after) {
		color: var(--color-blue);
		transform: translate3d(calc(-1 * var(--moveOffset)), calc(-1 * var(--moveOffset)), 0);
		transform: translate3d(
			calc(-1 * var(--moveX, var(--moveOffset))),
			calc(-1 * var(--moveY, var(--moveOffset))),
			0
		);
	}
</style>
