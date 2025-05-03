"use client";

import { useState } from "react";
// import { Sidebar } from "@/components/sidebar"
import { Sidebar } from "../../components/sidebar";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
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
    <div className="bg-[#F5F4EF] min-h-screen">
      <Sidebar />

      {/* Main content */}
      <div className="md:ml-64 min-h-screen">
        {/* Page content with proper padding and centering */}
        <div className="p-6 md:p-10">
          <div className="max-w-3xl mx-auto">
            <h1 className="text-3xl font-bold text-[#573900] mb-8">
              Burn Your GlobeCoin Here 😫
            </h1>

            <div className="bg-white rounded-lg p-6 md:p-8 shadow-sm border border-gray-100">
              {/* Amount input */}
              <div className="mb-6">
                <Label
                  htmlFor="amount"
                  className="text-lg font-medium text-[#573900] mb-2 block"
                >
                  Amount to Burn:
                </Label>
                <div className="flex items-center gap-2">
                  <Input
                    id="amount"
                    type="number"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    className="border-[#573900]/20 focus-visible:ring-[#F1B62A]"
                    placeholder="Enter amount"
                  />
                  <span className="text-[#573900] font-medium">GC</span>
                </div>
              </div>

              <div className="mb-6">
                <Label className="text-lg font-medium text-[#573900] mb-2 block">
                  Amount Get for Burn:
                </Label>
                <div className="flex gap-2">
                  <Input
                    readOnly
                    value={coinValue}
                    placeholder="Calculated amount"
                    className="border-[#573900]/20 bg-[#F5F4EF] text-gray-600 w-full"
                  />
                  <Select
                    value={selectedCurrency}
                    onValueChange={setSelectedCurrency}
                  >
                    <SelectTrigger className="border-[#573900]/20 w-40">
                      <SelectValue placeholder="Currency" />
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
              </div>

              <div className="mb-8">
                <Label
                  htmlFor="bank"
                  className="text-lg font-medium text-[#573900] mb-2 block"
                >
                  Bank Account to Receive Money:
                </Label>
                <Input
                  id="bank"
                  type="text"
                  placeholder="Enter your bank account"
                  className="border-[#573900]/20"
                />
              </div>

              {/* Action buttons */}
              <div className="flex flex-wrap gap-4">
                <Button
                  className="bg-[#F1B62A] text-[#573900] hover:bg-[#F1B62A]/90 px-6 py-2 text-base font-medium"
                  onClick={handleCalculate}
                >
                  Calculate
                </Button>
                <Button
                  variant="outline"
                  className="border-[#573900] text-[#573900] hover:bg-[#F1B62A]/20 px-6 py-2 text-base font-medium"
                >
                  Burn
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
