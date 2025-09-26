<template>

  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="/src/assets/styles.css" rel="stylesheet">
  </head>

  <body>
    <div class="w-full min-h-[calc(100dvh)] bg-[#0d0721]">
      <div class="w-full px-[20px] pt-[20px]">
        <h1 class="text-white text-[26px] font-bold">True Rate</h1>
        <div class="flex items-center">
          <h1 class="text-white text-[26px] font-bold">Designed for</h1>
          <img src="/img/Wise_Logo_512x124.svg?url." class="h-[22px] ml-[7px]"></img>
        </div>
        <h4 class="w-full flex justify-center text-white text-[22px] font-bold  pt-[50px]">
          Origin
        </h4>
        <!-- Origin Input -->
        <div class="w-full flex justify-center pt-[20px]">
          <div class="relative w-[100px]">
            <!-- Dropdown trigger -->
            <div class="w-[100px] h-[40px] bg-white/10 rounded-l-[10px] flex items-center px-[10px] cursor-pointer"
              @click="toggleDropdown('origin')">
              <div class="flex items-center">
                <i class="rounded-[3px]" :class="currencies.find(currencies => currencies.code === origin_currency.code)?.icon"></i>
                <h4 class="text-white font-bold ml-[5px] select-none">{{ origin_currency.code }}</h4>
              </div>
              <div class="w-full text-right">
                <i class="fas fa-angle-down text-white transition-transform duration-200"
                  :class="{ 'rotate-180': C_dropdown.find(i => i.where === 'origin')?.isOpen }"></i>
              </div>
            </div>

            <!-- Dropdown menu -->
            <div v-if="C_dropdown.find(i => i.where === 'origin')?.isOpen"
              class="max-h-[300px] absolute top-[45px] left-0 w-full bg-white/20 rounded-[10px] select-none overflow-scroll scrollbar-hide shadow-md backdrop-blur-sm z-10">
              <ul>
                <li v-for="(currency, index) in currencies" :key="index" @click="selectCurrency(currency, 'origin'), toggleDropdown('origin')"
                  class="flex items-center px-[10px] py-[8px] cursor-pointer hover:bg-white/30 rounded-[8px]">
                  <i :class="currency.icon + ' rounded-[3px]'"></i>
                  <span class="text-white font-medium ml-[5px]">{{ currency.code }}</span>
                </li>
              </ul>
            </div>
          </div>
          <!-- Origin Input -->
          <input v-model="origin_currency.amount" @change="Calculator('origin', origin_currency.amount)" class="w-[300px] h-[40px] bg-white/10 rounded-r-[10px] outline-none text-white font-bold" placeholder="amount" type="text"></input>
        </div>

        <h4 class="w-full flex justify-center text-white text-[22px] font-bold  pt-[50px]">
          Targets
        </h4>
        <!-- Target Inputs -->
         <div class="w-full grid gap-y-[20px] pt-[20px]" :class="{'grid-cols-3': target_currencies.length >= 3, 'grid-cols-2': target_currencies.length === 2, 'grid-cols-1 place-items-center': target_currencies.length === 1}">
          <div v-for="i in target_currencies" class="w-full flex justify-center pt-[20px]" :class="{'ml-[12px]': target_currencies.length >= 3 && C_dropdown[C_dropdown.length - 1].where === i.code}">
            <div class="relative w-[100px]">
              <!-- Dropdown trigger -->
              <div class="w-[100px] h-[40px] bg-white/10 rounded-l-[10px] flex items-center px-[10px] cursor-pointer"
                @click="toggleDropdown(i.code)">
                <div class="flex items-center">
                  <i class="rounded-[3px]" :class="currencies.find(currencies => currencies.code === i.code)?.icon"></i>
                  <h4 class="text-white font-bold ml-[5px] select-none">{{ i.code }}</h4>
                </div>
                <div class="w-full text-right">
                  <i class="fas fa-angle-down text-white transition-transform duration-200"
                    :class="{ 'rotate-180': C_dropdown.find(dropdown => dropdown.where === i.code)?.isOpen }"></i>
                </div>
              </div>

              <!-- Dropdown menu -->
              <div v-if="C_dropdown.find(dropdown => dropdown.where === i.code)?.isOpen"
                class="max-h-[300px] absolute top-[45px] left-0 w-full bg-white/20 rounded-[10px] select-none overflow-scroll scrollbar-hide shadow-md backdrop-blur-sm z-10">
                <ul>
                  <li v-for="(currency, index) in currencies" :key="index" @click="selectCurrency(currency, i.code), toggleDropdown(i.code)"
                    class="flex items-center px-[10px] py-[8px] cursor-pointer hover:bg-white/30 rounded-[8px]">
                    <i :class="currency.icon + ' rounded-[3px]'"></i>
                    <span class="text-white font-medium ml-[5px]">{{ currency.code }}</span>
                  </li>
                </ul>
              </div>
            </div>
            <!-- Origin Input -->
            <input v-model="i.amount" @change="Calculator(i.code, i.amount)" class="w-[200px] h-[40px] bg-white/10 rounded-r-[10px] outline-none text-white font-bold" placeholder="amount" type="text"></input>
            
            <!-- Add Target Button -->
            <button v-show="target_currencies[target_currencies.length - 1].code === i.code" @click="addTargetCurrency()" class="flex items-center text-white/50 text-[20px]">
              <i class="far fa-plus-circle ml-[5px]"></i>
            </button>
          </div>
        </div>
      </div>
    </div>
  </body>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  warn,
  debug,
  trace,
  info,
  error,
  attachConsole,
  attachLogger,
} from '@tauri-apps/plugin-log';
import { Currency } from './interface/types.ts';

let C_dropdown = ref([{ where: "origin", isOpen: false }, { where: "AUD", isOpen: false }]);
let origin_currency = ref<Currency>({ code: "USD", amount: null as number | null });
let target_currencies = ref<Currency[]>([{ code: "AUD", amount: null as number | null }]);

const currencies = [
  { code: "AUD", icon: "fi fi-au" },
  { code: "BGN", icon: "fi fi-bg" },
  { code: "BRL", icon: "fi fi-br" },
  { code: "CAD", icon: "fi fi-ca" },
  { code: "CHF", icon: "fi fi-ch" },
  { code: "CNY", icon: "fi fi-cn" },
  { code: "CZK", icon: "fi fi-cz" },
  { code: "DKK", icon: "fi fi-dk" },
  { code: "EUR", icon: "fi fi-eu" },
  { code: "GBP", icon: "fi fi-gb" },
  { code: "HKD", icon: "fi fi-hk" },
  { code: "HUF", icon: "fi fi-hu" },
  { code: "IDR", icon: "fi fi-id" },
  { code: "ILS", icon: "fi fi-il" },
  { code: "INR", icon: "fi fi-in" },
  { code: "JPY", icon: "fi fi-jp" },
  { code: "MYR", icon: "fi fi-my" },
  { code: "MXN", icon: "fi fi-mx" },
  { code: "NOK", icon: "fi fi-no" },
  { code: "NZD", icon: "fi fi-nz" },
  { code: "PHP", icon: "fi fi-ph" },
  { code: "PLN", icon: "fi fi-pl" },
  { code: "RON", icon: "fi fi-ro" },
  { code: "SEK", icon: "fi fi-se" },
  { code: "SGD", icon: "fi fi-sg" },
  { code: "TRY", icon: "fi fi-tr" },
  { code: "USD", icon: "fi fi-us" },
  { code: "ZAR", icon: "fi fi-za" },
  { code: "UGX", icon: "fi fi-ug" },
  { code: "AED", icon: "fi fi-ae" },
  { code: "CNY", icon: "fi fi-cn" },
];

async function Calculator(where: string, raw_amount: number | null) {
  if (raw_amount === null) {
    warn(`Raw amount is null for ${where}.`);
    return;
  }
  const result: number = new Function("return " + raw_amount.toString())();
  if (where === 'origin') {
    origin_currency.value.amount = result;
  } else {
    const target = target_currencies.value.find(i => i.code === where);
    if (target) {
      target.amount = result;
    } else {
      warn(`Target currency with code="${where}" not found.`);
    }
  }
  debug(`Calculated amount for ${where}: ${raw_amount} = ${result}${where}`);

  // Call the Rust command
  const response = await invoke<number>("greet", {
    origincurrency: origin_currency.value,
    targetcurrencies: target_currencies.value
  });
  debug(`Response from Rust: ${response}`);
}

function toggleDropdown(where: string) {
  const dropdown = C_dropdown.value.find(i => i.where === where);
  if (dropdown) {
    dropdown.isOpen = !dropdown.isOpen;
  } else {
    console.warn(`Dropdown with where="${where}" not found.`);
  }
}

function selectCurrency(currency: { code: string; icon: string }, where: string) {
  if (where === 'origin') {
    origin_currency.value.code = currency.code;
  } else if (where === target_currencies.value.find(i => i.code === where)?.code) {
    const target = target_currencies.value.find(i => i.code === where);
    if (target) {
      target.code = currency.code;
      C_dropdown.value.find(i => i.where === where)!.where = currency.code;
    } else {
      warn(`Target currency with code="${where}" not found.`);
    }
  } else {
    warn(`Where="${where}" does not match any known currency slots.`);
  }
}

function addTargetCurrency() {
  //random currency not already in target_currencies and push it to target_currencies
  let filltered_currencies = currencies.filter(currency => !target_currencies.value.some(target => target.code === currency.code) && currency.code !== origin_currency.value.code);
  if (filltered_currencies.length === 0) {
    warn("No more currencies to add.");
    return;
  } else {
    let random_currency = filltered_currencies[Math.floor(Math.random() * filltered_currencies.length)];
    target_currencies.value.push({ code: random_currency.code, amount: null });
    C_dropdown.value.push({ where: random_currency.code, isOpen: false });
    debug(`Added target currency: ${random_currency.code}`);
  }
}
</script>
