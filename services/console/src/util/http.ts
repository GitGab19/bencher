import axios from "axios";

enum HttpMethod {
	GET = "GET",
	POST = "POST",
	PUT = "PUT",
	PATCH = "PATCH",
	DELETE = "DELETE",
}

export const httpGet = async (url: string, token: undefined | null | string) =>
	axios(getOptions(url, token));
export const getOptions = (url: string, token: undefined | null | string) => {
	return {
		url: url,
		method: HttpMethod.GET,
		headers: headers(token),
	};
};

export const httpPost = async (
	url: string,
	token: undefined | null | string,
	data: {},
) => axios(postOptions(url, token, data));
export const postOptions = (
	url: string,
	token: undefined | null | string,
	data: {},
) => {
	return dataOptions(url, HttpMethod.POST, token, data);
};

export const httpPut = async (
	url: string,
	token: undefined | null | string,
	data: {},
) => axios(putOptions(url, token, data));
export const putOptions = (
	url: string,
	token: undefined | null | string,
	data: {},
) => {
	return dataOptions(url, HttpMethod.PUT, token, data);
};

export const httpPatch = async (
	url: string,
	token: undefined | null | string,
	data: {},
) => axios(pathOptions(url, token, data));
export const pathOptions = (
	url: string,
	token: undefined | null | string,
	data: Record<string, any>,
) => {
	return dataOptions(url, HttpMethod.PATCH, token, data);
};

export const httpDelete = async (
	url: string,
	token: undefined | null | string,
) => axios(deleteOptions(url, token));
export const deleteOptions = (
	url: string,
	token: undefined | null | string,
) => {
	return {
		url: url,
		method: HttpMethod.DELETE,
		headers: headers(token),
	};
};

const HEADERS_CONTENT_TYPE = {
	"Content-Type": "application/json",
};

const headers = (token: undefined | null | string) => {
	if (token) {
		return {
			...HEADERS_CONTENT_TYPE,
			Authorization: `Bearer ${token}`,
		};
	} else {
		return HEADERS_CONTENT_TYPE;
	}
};

export const dataOptions = (
	url: string,
	method: HttpMethod,
	token: undefined | null | string,
	data: {},
) => {
	return {
		url: url,
		method: method,
		headers: headers(token),
		data: data,
	};
};
