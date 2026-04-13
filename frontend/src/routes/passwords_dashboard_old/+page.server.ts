import { fail, redirect, type Actions } from "@sveltejs/kit";

export interface PasswordEntry {
    id: string,
    origin: string,
    username : string,
    password : Array<number>,
    salt : Array<number>,
    added_time : number,
}

export interface PasswordEntryClean {
    id: string,
    origin: string,
    username : string,
    password : string,
    added_time : string,
}

function decode_password(encodedData: number[]): string {
    return new TextDecoder().decode(new Uint8Array(encodedData));
}

function parseJwt(token : string) : Record<string, any> {
    var base64Url = token.split('.')[1];
    var base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    var jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));

    return JSON.parse(jsonPayload);
}

export async function load({ cookies, fetch })  {
    let jwt_token = cookies.get("jwt_token");

    if (!jwt_token) {
        redirect(303, "/")
    }

    let exp : number = parseJwt(jwt_token)["exp"]

    if (exp < Math.floor(Date.now() / 1000)) {
        redirect(303, "/")
    } 
    
    console.log(parseJwt(jwt_token))

    let response = await fetch("http://127.0.0.1:3000/get_entry_list", {
        method : "GET",
        headers : {
            "Authorization" : jwt_token
        }
    });

    console.log(response);
    
    let entries : Array<PasswordEntry> = await response.json();
    let entriesClean : Array<PasswordEntryClean> = entries.map((entry) => {
        return {
            id :  entry.id,
            origin : entry.origin,
            username : entry.username,
            password : decode_password(entry.password),
            added_time : new Date(entry.added_time).toLocaleString(),
        }
    });
    console.log(entriesClean);
    return { entries : entriesClean };
};

export const actions: Actions = {
    default: async ({ request, cookies }) => {
        const data = await request.formData();
        
        let password = data.get("password")?.toString();
        let username = data.get("username")?.toString();
        let origin = data.get("origin")?.toString();

        let errors : Record<string, string | undefined> = {
            "password" : undefined,
            "username" : undefined,
            "origin" : undefined,
        }

        if (!password) {
            errors["password"] = "Enter a password."
        }

        if (!username) {
            errors["username"] = "Enter a username."
        }

        if (!origin) {
            errors["origin"] = "Enter a origin."
        }

        const hasErrors = (errors["password"] != undefined) || (errors["username"] != undefined) || (errors["origin"] != undefined);

        if (hasErrors) {
            return fail(
                400,
                { 
                    success : false,
                    errors
                }
            )
        }

        let jwt_token = cookies.get("jwt_token");

        if (!jwt_token) {
            redirect(303, "/")
        }

        let exp : number = parseJwt(jwt_token)["exp"]

        if (exp < Math.floor(Date.now() / 1000)) {
            redirect(303, "/")
        } 
        
        let password_as_bytes = [...(new TextEncoder().encode(password))]
        
        let entry : PasswordEntry = {
            id: "",
            origin: origin || "",
            username : username || "",
            password : password_as_bytes,
            salt : new Array(16).fill(0), 
            added_time: Date.now(),
        }

        let response = await fetch("http://127.0.0.1:3000/add_entry", {
            method : "POST",
            headers : {
                "Authorization" : jwt_token,
                "Content-Type": "application/json"
            },
            body : JSON.stringify(entry)
        });

        if (!response.ok) {
            return { success : false }
        }

        return { success : true, entry }
    }
}