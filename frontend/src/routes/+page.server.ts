import type { PageServerLoad } from './$types';
import { fail, redirect, type Actions } from '@sveltejs/kit';

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

/**
 * Parses a raw Cookie string into a key-value object.
 * @param rawCookies - The string from document.cookie or request headers.
 */
function parseCookies(rawCookies: string): Record<string, string>  {
  if (!rawCookies) return {};

  return rawCookies
    .split(';')
    .reduce((acc: Record<string, string>, cookieString: string) => {
      // Split by the first '=' found
      const [key, ...valueParts] = cookieString.split('=');
      
      if (key) {
        const trimmedKey = key.trim();
        // Join value parts back in case the value itself contains an '='
        const value = valueParts.join('=').trim();
        
        // Only add to object if key is not empty
        if (trimmedKey) {
          acc[trimmedKey] = decodeURIComponent(value);
        }
      }
      
      return acc;
    }, {});
};

export const actions: Actions = {
    default: async ({ request, cookies }) => {
        const data = await request.formData();

        const isSigningIn = data.get('isSigningIn')?.toString() === "true"; 
        const username = data.get('username')?.toString();
        const password = data.get('password')?.toString();

        const errors: Record<string, Array<string>> = {
            "username" : [],
            "password" : [],
            "message" : []
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

        let url = ""
        let body = {}

        console.log("isSigningIn : ", isSigningIn);

        if (isSigningIn) {
            url = "http://127.0.0.1:3000/login"
            body = {
                username: username,
                master_password: password,
                exp : 30 * 60 
            }  
        } else {
            url = "http://127.0.0.1:3000/create_user"
            body = {
                id : "",
                username: username,
                hashed_master_password: password,
                passwords : [],
            }  
        }

        const response = await fetch(url, { 
            method : "POST",
            body : JSON.stringify(body),
            headers : {
                "Content-Type" : "application/json"
            }
        });

        
        console.log(response)
        console.log("Body : ", await response.text())

        if (!response.ok) {
            if (!isSigningIn && response.status === 406) { // if the user try to create an account with an already used username
                errors.username.push("This username already exists, please enter another one.");
                return fail(400, { 
                    errors, 
                    username : "",
                });
            } else if (isSigningIn && response.status === 404) { // if the user try to sign in but with a username that doesn't exists
                errors.username.push("This user doesn't exists.");
                return fail(400, { 
                    errors, 
                    username : "",
                });
            } else if (isSigningIn && response.status === 403) { // when the user try to sign in with a invalid credentials 
                errors.password.push("Wrong password.");
                return fail(400, { 
                    errors, 
                    username : username,
                });
            }
            return fail(response.status, { 
                serverError: "Backend connection failed",
                username 
            });
        }

        let parsedCookies = parseCookies(response.headers.getSetCookie()[0])

        console.log(parsedCookies["token"]) 

        cookies.set("jwt_token", parsedCookies["token"], {path : "/"})

        if (isSigningIn) {
            console.log("Successfully signed in")
        } else {
            console.log("Account Successfully created")
        }

        redirect(303, "/passwords_dashboard")
    }
};