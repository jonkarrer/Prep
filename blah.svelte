<script>
	import { goto } from '$app/navigation';
	import LogoutOutlined from '$lib/assets/icons/LogoutOutlined.svelte';
	import SaveTwoTone from '$lib/assets/icons/SaveTwoTone.svelte';
	import Button from '$lib/components/common/Button.svelte';
	import Mobile from '$lib/components/headers/Mobile.svelte';
	import Send from '$lib/helpers/Send';
	import { PROXY_ROUTES } from '$lib/types/Enums';

	async function logout() {
		const request = new Request(PROXY_ROUTES.LOGOUT_USER);
		Send(request);

		goto('/auth/login');
	}
</script>

<Mobile />

<main>
	<section>
		<div class="profile">
			<div class="profile-img-container">
				<img class="profile-img" src="/profiles/carrot.png" alt="carrot icon" />
			</div>
			<div class="profile-name-container">
				<h1 class="profile-name">username</h1>
				<h2 class="profile-username">@handle</h2>
			</div>
		</div>
		<form class="form">
			<label for="username">Username</label>
			<input type="text" id="username" placeholder="yourusername" />
			<label for="email">Email</label>
			<input type="text" id="email" placeholder="email@email.com" />
		</form>
		<div class="profile-buttons">
			<Button
				text="Update"
				icon={SaveTwoTone}
				toggle={false}
				callback={() => console.log('update')}
			/>
			<Button text="Log Out" icon={LogoutOutlined} toggle={false} callback={logout} />
		</div>
	</section>
</main>

<style>
	main {
		margin-top: 130px;
	}

	.profile {
		margin-left: 30px;

		display: flex;
		align-items: flex-end;
		gap: 15px;
	}

	.profile-img {
		width: 120px;
		height: 150px;

		border: 1px solid var(--contrast);
		border-radius: 3px;
	}

	.profile-img-container {
		position: relative;
		top: -30px;
	}

	.profile-name-container {
		margin-bottom: 60px;
	}

	.form {
		margin-left: 30px;

		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.profile-buttons {
		margin: 30px 0 160px 30px;

		display: flex;
		gap: 15px;
	}
	h1,
	h2 {
		font-weight: 500;
	}

	h1 {
		color: var(--accent);
		font-size: 24px;
	}

	h2 {
		font-size: 14px;
	}

	label {
		font-size: 20px;
		font-weight: 400;
	}

	input {
		width: 310px;
		height: 40px;

		margin: 0;

		padding-left: 10px;

		color: var(--darker);
		font-family: 'Bai Jamjuree';
		background-color: var(--primary);
		border: 1px solid var(--contrast);
		border-radius: 3px;
	}
</style>