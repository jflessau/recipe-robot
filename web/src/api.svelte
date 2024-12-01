<script lang="ts" context="module">
	import { camelizeKeys, decamelizeKeys } from 'humps';
	import axios, { type AxiosInstance } from 'axios';

	let baseUrl: string | boolean = import.meta.env.VITE_API_BASE_URL_LOCAL;
	if (import.meta.env.MODE === 'production') {
		baseUrl = import.meta.env.VITE_API_BASE_URL_REMOTE;
	}

	export const apiUrl: string = baseUrl.toString();

	class Client {
		client: AxiosInstance;

		constructor() {
			let client = axios.create({
				baseURL: `${baseUrl}`,
				withCredentials: true
			});

			client.interceptors.request.use((config) => {
				const newConfig = { ...config };
				newConfig.url = `${baseUrl}${config.url}`;
				if (config.params) {
					newConfig.params = decamelizeKeys(config.params);
				}
				if (config.data) {
					newConfig.data = decamelizeKeys(config.data);
				}

				return newConfig;
			});

			client.interceptors.response.use(
				(response) => {
					if (response.data && response.headers['content-type'] === 'application/json') {
						response.data = camelizeKeys(response.data);
					}
					return response;
				},
				(error) => {
					const { response } = error;
					return this.handleError(response);
				}
			);

			this.client = client;
		}

		/* eslint-disable */
		handleError(error: any) {
			return Promise.reject(error);
		}
		/* eslint-enable */

		async join<T = { data: { username: string; password: string }; status: number }>(inviteCode: string): Promise<T> {
			return this.client.post(`/auth/join`, { inviteCode });
		}

		async login<T = { status: number }>(payload: { username: string; password: string }): Promise<T> {
			return this.client.post(`/auth/login`, payload);
		}

		async me<
			T = {
				status: number;
				data: Me;
			}
		>(): Promise<T> {
			return this.client.get(`/auth/me`);
		}

		async recipeIngredients<T = { status: number; data: { ingredients: Ingredient[] } }>(recipe: string): Promise<T> {
			return this.client.post(`/recipe/ingredients`, { text: recipe });
		}

		async ingredientItems<T = { status: number; data: Ingredient }>(ingredient: Ingredient): Promise<T> {
			return this.client.post(`/ingredient/items`, { ingredient });
		}

		async logout<T = { status: number }>(): Promise<T> {
			return this.client.get(`/auth/logout`);
		}
	}

	export const Api = new Client();

	// types

	export interface Me {
		username: string;
		percentageOfDailyLimit: number;
	}

	export interface Ingredient {
		id: string;
		name: string;
		probablyAtHome: boolean;
		unit: string;
		quantity: number;
		item: Item | null;
		itemQuantity: number;
		alternatives: Item[];
	}

	export interface Item {
		id: string;
		name: string;
		grammage: string | null;
		priceCent: number | null;
		url: string | null;
		imageUrl: string | null;
	}
</script>
