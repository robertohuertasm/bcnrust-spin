import type { UserModel } from './userModel';

let url = 'https://bcnrust-spin.fermyon.app/api/v1/users';
// let url = 'http://localhost:3000/api/v1/users';

export async function getUsers(): Promise<UserModel[]> {
  try {
    const res = await fetch(url);
    const users: UserModel[] = await res.json();
    return users.sort((a, b) => a.email.localeCompare(b.email));
  } catch (error) {
    console.log(error);
  }
  return [];
}

export async function getWinner(): Promise<UserModel | undefined> {
  try {
    const res = await fetch(`${url}/winner`);
    const winner = await res.json();
    return winner;
  } catch (error) {
    console.log(error);
  }
  return undefined;
}

export async function createUser(
  user: UserModel,
): Promise<UserModel | undefined> {
  try {
    const res = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(user),
    });
    const createdUser = await res.json();
    return createdUser;
  } catch (error) {
    console.log(error);
  }
  return undefined;
}
