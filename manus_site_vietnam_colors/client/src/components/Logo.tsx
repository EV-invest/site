import React from "react";

interface LogoProps {
  className?: string;
  color?: string;
}

export default function Logo({ className = "w-12 h-12", color = "currentColor" }: LogoProps) {
  return (
    <svg 
      className={className} 
      viewBox="0 0 500 500" 
      fill="none" 
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* 3 характерных здания с диагональными крышами */}
      {/* Левое здание */}
      <path 
        d="M205 190 L205 152 L250 132 L250 190 Z" 
        fill={color} 
      />
      <path 
        d="M213 162 L223 162 L223 182 L213 182 Z" 
        fill="#011F52" 
        opacity="0.9"
      />
      <path 
        d="M232 158 L242 158 L242 182 L232 182 Z" 
        fill="#011F52" 
        opacity="0.9"
      />

      {/* Центральное здание (доминирующее) */}
      <path 
        d="M266 198 L266 110 L302 124 L302 198 Z" 
        fill={color} 
      />
      {/* Теневая сторона центрального здания для 3D-эффекта как в оригинале */}
      <path 
        d="M302 124 L326 142 L326 198 L302 198 Z" 
        fill={color} 
        opacity="0.8"
      />

      {/* Правое здание */}
      <path 
        d="M338 195 L338 155 L354 162 L354 195 Z" 
        fill={color} 
      />

      {/* Дугообразная линия горизонта (горизонтальная линия, уходящая на спад к краям) */}
      <path 
        d="M115 208 C200 192, 300 192, 402 208 L402 202 C300 186, 200 186, 115 202 Z" 
        fill={color} 
      />

      {/* Массивный шрифт EV */}
      <text 
        x="250" 
        y="315" 
        fill={color} 
        fontSize="122" 
        fontWeight="900" 
        fontFamily="'Space Grotesk', 'Inter', sans-serif" 
        textAnchor="middle" 
        letterSpacing="-3"
      >
        EV
      </text>

      {/* INVESTMENT под ним */}
      <text 
        x="250" 
        y="360" 
        fill={color} 
        fontSize="24" 
        fontWeight="bold" 
        fontFamily="'Space Grotesk', 'Inter', sans-serif" 
        textAnchor="middle" 
        letterSpacing="11"
      >
        INVESTMENT
      </text>
    </svg>
  );
}
