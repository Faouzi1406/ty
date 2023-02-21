type AuthMessage<T> = {
  type: 'created' | 'delete' | 'update' | 'get' | 'error',
  payload: T,
}

export type User = {
  id: String,
  username: String,
  email: String,
}

export default class Auth {
  createSession(session: string) {
    const create = localStorage.setItem('session', session);
  }

  getUser(): Promise<AuthMessage<User | String>> {
    const getUser = async ():Promise<AuthMessage<User | String>> => {
      const session = localStorage.getItem('session');
      console.log();

      const user = await fetch('http://localhost:8080/users/auth/get_session_info', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json'
        },
        body: JSON.stringify(session)
      });

      if (user.ok) {
        const userInfo: User = await user.json();
        return { type: 'get', payload: { username: userInfo.username, email: userInfo.email, id: userInfo.id } };
      }
      else {
        return { type: 'error', payload: 'User not found' };
      }
    }

    return Promise.resolve(getUser());
  }
}
