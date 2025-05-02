// app/mint/page.tsx
"use client";

import { useState } from "react";
import { AppSidebar } from "@/components/sidebar/app-sidebar";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export default function MintPage() {
  const [amount, setAmount] = useState("");
  const [selectedCurrency, setSelectedCurrency] = useState("");
  const [coinValue, setCoinValue] = useState("");

  const currencies = ["MYR", "USD", "JPY", "YUAN"];

  const handleCalculate = () => {
    // Just for demo
    if (amount && selectedCurrency) {
      setCoinValue("123.45");
    } else {
      setCoinValue("");
    }
  };

  return (
    <SidebarProvider>
      <div className="flex min-h-screen bg-[#F5F4EF]">
        <AppSidebar />
        
        {/* Main content */}
        <div className="flex-1 bg-[#F5F4EF] text-[#573900]">
          {/* Page header with sidebar trigger */}
          <div className="pl-4 pt-4">
            <SidebarTrigger />
          </div>
          
          {/* Content area */}
          <div className="w-full p-8">
            <div className="mb-6">
              <h1 className="text-2xl md:text-3xl font-bold">
                <span className="inline-block mr-2">📄</span> Mint GlobeCoin
              </h1>
            </div>
            
            <div className="w-full bg-white rounded-lg p-8 shadow-sm border border-gray-100">
              {/* Amount */}
              <div className="mb-6">
                <Label htmlFor="amount" className="text-base block mb-2">
                  Amount to deposit:
                </Label>
                <Input
                  id="amount"
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  className="w-full"
                  placeholder="Enter amount"
                />
              </div>

              {/* Currency Dropdown */}
              <div className="mb-6">
                <Label className="text-base block mb-2">
                  Currency to Deposit:
                </Label>
                <Select 
                  value={selectedCurrency} 
                  onValueChange={setSelectedCurrency}
                >
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="Select a currency" />
                  </SelectTrigger>
                  <SelectContent>
                    {currencies.map((currency) => (
                      <SelectItem key={currency} value={currency}>
                        {currency}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              {/* Coin value */}
              <div className="mb-8">
                <Label className="text-base block mb-2">
                  Coin for this amount value:
                </Label>
                <div className="border px-4 py-2 rounded bg-gray-50 w-full">
                  {coinValue || "—"}
                </div>
              </div>

              {/* Buttons */}
              <div className="flex gap-4">
                <Button
                  className="bg-[#F1B62A] text-[#573900] hover:bg-[#d8a021]"
                  onClick={handleCalculate}
                >
                  Calculate
                </Button>
                <Button
                  variant="outline"
                  className="border-[#573900] text-[#573900] hover:bg-[#F1B62A]/20"
                >
                  Mint
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </SidebarProvider>
  );
}