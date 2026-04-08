import type { PageServerLoad } from './$types';
import { fail, type Actions } from '@sveltejs/kit';

export const load = (async () => {
    return {};
}) satisfies PageServerLoad;

function isUpperCase(ch: string): boolean {
  if (ch.length !== 1) return false;
  // if not a letter, treat as false
  if (ch.toLowerCase() === ch.toUpperCase()) return false;
  return ch === ch.toUpperCase();
}

function isLowerCase(ch: string): boolean {
  if (ch.length !== 1) return false;
  if (ch.toLowerCase() === ch.toUpperCase()) return false;
  return ch === ch.toLowerCase();
}

function countLowercase(s : string) : number {
    let count = 0
    for (const ch of s) {
        if (isLowerCase(ch)) {
            count++;
        }
    }

    return count;
}

function countUppercase(s : string) : number {
    let count = 0
    for (const ch of s) {
        if (isUpperCase(ch)) {
            count++;
        }
    }

    return count;
}

function countNumber(s : string) : number {
    let count = 0
    const number =  ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    for (const ch of s) {
        if (number.includes(ch)) {
            count++;
        }
    }

    return count;
}

export const actions: Actions = {
    default: async ({ request }) => {
        const data = await request.formData();
        const username = data.get('username')?.toString();
        const password = data.get('password')?.toString();

        const errors: Record<string, Array<string>> = {
            "username" : [],
            "password" : []
        };

        if (!username) {
            errors.username.push("Please enter a valid username or email");
        }

        if (!password) {
            errors.password.push("Enter a valid Password.");
            return fail(400, { 
                errors, 
                username,
            });
        } 

        if (password.length < 8) {
            errors.password.push("Password must be at least 8 characters.");
        }

        if (countLowercase(password) < 1) {
            errors.password.push("Password must contain at least 1 lowercase character.");
        }

        if (countUppercase(password) < 1) {
            errors.password.push("Password must contain at least 1 uppercase character.");
        }

        if (countNumber(password) < 1) {
            errors.password.push("Password must contain at least 1 number.");
        }

        // If there are errors, return a 400 status with the errors object
        const hasErrors = Object.values(errors).some(msgArray => msgArray.length > 0);

        if (hasErrors) {
            return fail(400, { 
                errors, 
                username,
            });
        }

        const body = {
            id : "",
            username: username,
            hashed_master_password: password,
            passwords: []
        }               

        const response = await fetch('http://127.0.0.1:3000/create_user', { 
            method : "POST",
            body : JSON.stringify(body),
            headers : {
                "Content-Type" : "application/json"
            }
        });

        console.log(response)

        if (!response.ok) {
            const result = await response.json(); 
                return fail(response.status, { 
                    serverError: result.message || "Backend connection failed",
                    username 
                });
        }

        return { success: true };
    }
};