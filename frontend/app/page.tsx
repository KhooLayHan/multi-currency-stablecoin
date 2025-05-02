"use client";


import Image from "next/image";
import { useRouter } from "next/navigation"; // 导入 App Router 的 useRouter
import { Button } from "@/components/ui/button"; // 导入 shadcn 的 Button 组件

export default function Home() {
  const router = useRouter(); // 初始化 router

  return (
    <div className="flex flex-col justify-center items-center min-h-screen px-8 bg-[#F5F4EF]">
      {/* 背景色：浅米白 */}

      {/* Hero Title */}
      <h1 className="text-4xl sm:text-6xl font-bold text-[#573900] mb-4 text-center">
        {/* 改成你的主标题文字 */}
        Welcome to GlobeCoin
      </h1>

      {/* Subheading */}
      <p className="text-lg sm:text-xl text-[#573900] mb-8 text-center max-w-xl">
        {/* 改成你的小标题介绍 */}
        A globally balanced multi-currency stablecoin for the decentralized world.
      </p>

      {/* Call-to-Action Button（shadcn样式按钮，点击跳转） */}
      <Button
        onClick={() => router.push("/Mint")} // 点击后跳转到 /mint 页面
        className="bg-[#F1B62A] text-[#573900] hover:brightness-110 font-semibold text-base sm:text-lg px-6 py-3 rounded-full transition-colors duration-200"
      >
        {/* 改成按钮上的文字 */}
        Connect Wallet
      </Button>
    </div>
  );
}
