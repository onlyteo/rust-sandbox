import {ClientResponse} from "../types";
import {basicAuthHeader} from "./principal";

export async function GET<T>(url: string): Promise<ClientResponse<T>> {
    return request<T>(url, {method: "GET"});
}

export async function POST<T, U>(url: string, body: U): Promise<ClientResponse<T>> {
    return request<T>(url, {method: "POST", body: JSON.stringify(body)});
}

export async function PUT<T, U>(url: string, body: U): Promise<ClientResponse<T>> {
    return request<T>(url, {method: "PUT", body: JSON.stringify(body)});
}

async function request<T>(url: string, init: RequestInit): Promise<ClientResponse<T>> {
    const response = await fetch(url, {
        ...init,
        headers: {
            "Accept": "application/json",
            "Content-Type": "application/json;charset=UTF-8",
            "Authorization": basicAuthHeader()
        },
        redirect: "manual"
    });
    return handleResponse<T>(response);
}

async function handleResponse<T>(response: Response): Promise<ClientResponse<T>> {
    const {status} = response;
    if (status >= 400) {
        const error = await response.json().catch(() => undefined);
        return Promise.reject({status, error});
    } else if (status === 200 || status === 201) {
        const data = await response.json();
        return Promise.resolve({status, data});
    } else {
        return Promise.resolve({status});
    }
}
