// The user of the frontend (there is no login). The credentials are sent as Basic auth on every call.
export interface Principal {
    name: string,
    username: string,
    password: string
}

export const principal: Principal = {
    name: "Luke Skywalker",
    username: "skywalker",
    password: "T4t01ne"
};

export const basicAuthHeader = ({username, password}: Principal = principal): string =>
    `Basic ${btoa(`${username}:${password}`)}`;
