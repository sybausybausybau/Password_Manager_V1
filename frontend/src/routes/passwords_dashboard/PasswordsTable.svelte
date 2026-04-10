
<script lang="ts">
    import { goto } from "$app/navigation";

    let { token } = $props()

window.onload = async () => {
    await getPasswordEntries();
}

function decode_password_entry(encodedData: Uint8Array): string {
    const decoder = new TextDecoder();
    const decodedString = decoder.decode(encodedData);
    return decodedString;
}

async function getPasswordEntries(): Promise<String> {
    let response = await fetch("http://127.0.0.1:3000/get_entry_list", {
        method : "GET",
        headers : {
            "Authorization" : token?.value || token
        }
    });

    if (!response.ok) {
        if (response.status === 403) {
            goto("/")
        } else {
            // Send to error page
        }
    }

    /* let data = (await response.body?.getReader().read())?.value;
    data?.forEach(password_entry => {
        let decoded_password_entry = decode_password_entry(password_entry);
        console.log(decoded_password_entry);
        data?.
    }); */

    let data = await response.json();

    // data is an ARRAY now
    data.forEach((entry: any) => {
        console.log("password:", entry.password);
    });

    console.log(data);
    console.log(JSON.stringify(data))

    return JSON.stringify(data);
}



</script>

<!-- {await } -->


<div class="w-full flex justify-between items-center mb-3 mt-1 pl-3">
    <div>
        <h3 class="text-lg font-semibold text-slate-800">All your passwords</h3>
        <p class="text-slate-500">Overview of the invoices.</p>
    </div>
    <div class="ml-3">
        <div class="w-full max-w-sm min-w-50 relative">
        <div class="relative">
            <input
            class="w-full pr-11 h-10 pl-3 py-2 bg-white placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-400 shadow-sm focus:shadow-md"
            placeholder="Search for invoice..."
            />
            <button
            class="absolute h-8 w-8 right-1 top-1 my-auto px-2 flex items-center bg-white rounded "
            type="button"
            >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="3" stroke="currentColor" class="w-8 h-8 text-slate-600">
                <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
            </svg>
            </button>
        </div>
        </div>
    </div>
</div>
 
<div class="relative flex flex-col w-full h-full overflow-scroll text-gray-700 bg-white shadow-md rounded-lg bg-clip-border">
  <table class="w-full text-left table-auto min-w-max">
    <thead>
      <tr>
        <th class="p-4 border-b border-slate-300 bg-slate-50">
            <p class="block text-sm font-normal leading-none text-slate-500">
                Invoice Number
            </p>
        </th>
        <th class="p-4 border-b border-slate-300 bg-slate-50">
            <p class="block text-sm font-normal leading-none text-slate-500">
                Customer
            </p>
        </th>
        <th class="p-4 border-b border-slate-300 bg-slate-50">
            <p class="block text-sm font-normal leading-none text-slate-500">
                Amount
            </p>
        </th>
        <th class="p-4 border-b border-slate-300 bg-slate-50">
            <p class="block text-sm font-normal leading-none text-slate-500">
                Issued
            </p>
        </th>
        <th class="p-4 border-b border-slate-300 bg-slate-50">
            <p class="block text-sm font-normal leading-none text-slate-500">
                Due Date
            </p>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr class="hover:bg-slate-50">
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="block font-semibold text-sm text-slate-800">INV-1001</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="text-sm text-slate-500">John Doe</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="text-sm text-slate-500">$1,200.00</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="text-sm text-slate-500">2024-08-01</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="text-sm text-slate-500">2024-08-15</p>
        </td>
      </tr>
      <tr class="hover:bg-slate-50">
        <td class="p-4 border-b border-slate-200 py-5">
            <p class="block font-semibold text-sm text-slate-800">INV-1002</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">Jane Smith</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">$850.00</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">2024-08-05</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">2024-08-20</p>
        </td>
      </tr>
      <tr class="hover:bg-slate-50">
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="block font-semibold text-sm text-slate-800">INV-1003</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">Acme Corp</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">$2,500.00</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">2024-08-07</p>
        </td>
        <td class="p-4 border-b border-slate-200 py-5">
          <p class="text-sm text-slate-500">2024-08-21</p>
        </td>
      </tr>
      <tr class="hover:bg-slate-50">
        <td class="p-4 py-5">
          <p class="block font-semibold text-sm text-slate-800">INV-1004</p>
        </td>
        <td class="p-4 py-5">
          <p class="text-sm text-slate-500">Global Inc</p>
        </td>
        <td class="p-4 py-5">
          <p class="text-sm text-slate-500">$4,750.00</p>
        </td>
        <td class="p-4 py-5">
          <p class="text-sm text-slate-500">2024-08-10</p>
        </td>
        <td class="p-4 py-5">
          <p class="text-sm text-slate-500">2024-08-25</p>
        </td>
      </tr>
    </tbody>
  </table>
</div>