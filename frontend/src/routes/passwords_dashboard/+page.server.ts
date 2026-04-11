interface PasswordEntry {
    id: string,
    origin: string,
    username : string,
    password : Array<number>,
    salt : Uint8Array,
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

export async function load({ cookies, fetch })  {
    let token = cookies.get("jwt_token");

    console.log(token)

    let response = await fetch("http://127.0.0.1:3000/get_entry_list", {
        method : "GET",
        headers : {
            "Authorization" : token || ""
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
            added_time : new Date(entry.added_time * 1000).toLocaleString(),
        }
    });
    console.log(entriesClean);
    return { entries : entriesClean };
};