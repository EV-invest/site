import React from "react";
import { Link } from "wouter";
import Logo from "@/components/Logo";

export default function NotFound() {
  return (
    <div className="min-h-screen bg-[#111418] text-[#DAE2EF] flex flex-col items-center justify-center p-6 selection:bg-[#8EB8FE] selection:text-[#090B0F]">
      <div className="max-w-md w-full text-center space-y-8">
        <div className="flex justify-center">
          <Logo className="h-12" />
        </div>
        
        <div className="space-y-4">
          <h1 className="text-6xl font-mono font-bold text-[#8EB8FE]">404</h1>
          <h2 className="text-xl font-bold text-white tracking-wide uppercase">PAGE NOT FOUND</h2>
          <p className="text-sm text-[#979FAC] font-light leading-relaxed">
            The requested document or report is restricted or has been moved to another section of our secure database.
          </p>
        </div>

        <div className="pt-4">
          <Link href="/">
            <button className="px-6 py-3 bg-[#8EB8FE] text-[#090B0F] text-xs font-semibold tracking-widest hover:bg-white transition-all duration-300 cursor-pointer">
              RETURN TO OVERVIEW
            </button>
          </Link>
        </div>
      </div>
    </div>
  );
}
