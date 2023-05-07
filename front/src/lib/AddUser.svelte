<script lang="ts">
  import * as api from './api';
  import type { UserModel } from './userModel';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let users: UserModel[] = [];
  let userName = '';
  let userEmail = '';
  let isValid = false;

  $: {
    const trimmedName = userName.trim();
    const trimmedEmail = userEmail.trim();
    const rxEmail = /\S+@\S+\.\S+/;
    isValid =
      trimmedName.length && trimmedEmail.length && rxEmail.test(trimmedEmail);
  }

  async function refreshUsers() {
    users = await api.getUsers();
    dispatch('winner', undefined);
  }

  async function getWinner() {
    const winner = await api.getWinner();
    dispatch('winner', winner);
  }

  async function createUser() {
    const user = { name: userName, email: userEmail };
    const newUser = await api.createUser(user);
    if (newUser) {
      users = [...users, newUser];
    }
  }
</script>

<input type="text" bind:value={userName} placeholder="Enter a name" id="name" />
<input
  type="text"
  bind:value={userEmail}
  placeholder="Enter an email"
  id="email"
/>

<button on:click={createUser} disabled={!isValid}> + Add User </button>
<button on:click={refreshUsers}> Refresh Users </button>
<button on:click={getWinner}> Get Winner </button>
