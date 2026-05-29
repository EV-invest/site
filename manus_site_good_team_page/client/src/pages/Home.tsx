import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { 
  TrendingUp, 
  ArrowUpRight, 
  MapPin, 
  Shield, 
  Building, 
  ChevronRight, 
  Download, 
  BookOpen, 
  Globe, 
  Briefcase, 
  Layers, 
  ChevronDown,
  Info,
  ExternalLink,
  Users,
  Search,
  Filter
} from "lucide-react";
import { toast } from "sonner";

// URLs сжатых изображений
const HERO_IMAGE = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/quy_nhon_bay_hero-mAEsZDLe6ykxeMgDqJEqyB.webp";
const CHART_IMAGE = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/investment_chart_3d-ZMn7a9hR33jX8sFmDDZCnL.webp";
const TEAM_1 = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/team_member_1-hJnPwkoXB2TsmzqEqY9yWb.webp";
const TEAM_2 = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/team_member_2-cdQ8SpsdHCexzaZSqvgtaE.webp";
const OFFICE_IMAGE = "https://d2xsxph8kpxj0f.cloudfront.net/310519663075853325/FwrpPAwgLER3rBrQqC8HHt/office_interior-SxhsdYStJCspKeR4RjpDbW.webp";

// Объекты инвестирования (для интерактивного зума/выбора в стиле Stronghold)
const INVESTMENT_OBJECTS = [
  {
    id: "obj-1",
    title: "Quy Nhon Coastal Heights",
    type: "Премиальные апартаменты",
    location: "Полуостров Фыонгмай, Куинён",
    roi: "14.2% YTD",
    occupancy: "96%",
    minInvestment: "$250,000",
    coords: { x: "25%", y: "40%" },
    desc: "Эксклюзивный комплекс небоскребов на первой линии побережья с панорамным видом на залив Куинён."
  },
  {
    id: "obj-2",
    title: "The Horizon Cliff Villas",
    type: "Виллы на утесе",
    location: "Пляж Бай Ксеп, Куинён",
    roi: "18.5% YTD",
    occupancy: "100%",
    minInvestment: "$750,000",
    coords: { x: "65%", y: "60%" },
    desc: "Лимитированная коллекция вилл, интегрированных в скалистый ландшафт с прямым спуском к частному пляжу."
  },
  {
    id: "obj-3",
    title: "Thi Nai Lagoon Eco-Resort",
    type: "Экологический курорт",
    location: "Лагуна Тхи Най, Куинён",
    roi: "11.8% YTD",
    occupancy: "88%",
    minInvestment: "$150,000",
    coords: { x: "45%", y: "25%" },
    desc: "Устойчивый эко-курорт премиум-класса, сфокусированный на гармонии с природой и возобновляемых технологиях."
  }
];

// Аналитические отчеты (в стиле Bridgewater)
const RESEARCH_REPORTS = [
  {
    id: "rep-1",
    date: "Май 2026",
    category: "Макроэкономика Вьетнама",
    title: "Перспективы Quy Nhon как нового финансового и туристического хаба Юго-Восточной Азии",
    summary: "Глубокий анализ инфраструктурного развития провинции Биньдинь, расширения аэропорта Фукат и притока прямых иностранных инвестиций (FDI) в регион.",
    readTime: "12 мин чтения",
    fileSize: "4.2 MB"
  },
  {
    id: "rep-2",
    date: "Апрель 2026",
    category: "Рынок недвижимости",
    title: "Анализ доходности прибрежной недвижимости: Сравнение Дананга, Нячанга и Куинёна",
    summary: "Почему Quy Nhon демонстрирует опережающие темпы роста капитализации земельных участков и арендной платы благодаря контролируемому девелопменту.",
    readTime: "18 мин чтения",
    fileSize: "5.8 MB"
  },
  {
    id: "rep-3",
    date: "Март 2026",
    category: "Институциональные тренды",
    title: "Влияние регуляторных реформ Вьетнама на владение недвижимостью иностранными инвесторами",
    summary: "Обзор последних законодательных изменений, упрощающих транзакции и повышающих юридическую защиту суверенных и частных фондов.",
    readTime: "9 мин чтения",
    fileSize: "2.1 MB"
  }
];

export default function Home() {
  const [selectedObject, setSelectedObject] = useState(INVESTMENT_OBJECTS[0]);
  const [zoomLevel, setZoomLevel] = useState(1);
  const [isScrolled, setIsScrolled] = useState(false);
  const [activeTab, setActiveTab] = useState("dashboard");

  // Обработка скролла для изменения стилей хедера
  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 50) {
        setIsScrolled(true);
      } else {
        setIsScrolled(false);
      }
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const handlePlaceholderClick = (name: string) => {
    toast.info(`Интерактивный прототип: функция "${name}" будет подключена в финальной версии.`);
  };

  return (
    <div className="min-h-screen bg-[#060e1e] text-white overflow-x-hidden selection:bg-[#2A9D8F] selection:text-white">
      
      {/* ПРЕМИАЛЬНЫЙ ХЕДЕР (Симбиоз Stronghold / Binance) */}
      <header className={`fixed top-0 left-0 right-0 z-50 transition-all duration-500 ${
        isScrolled 
          ? "bg-[#060e1e]/90 backdrop-blur-xl border-b border-white/10 py-4 shadow-2xl" 
          : "bg-transparent py-6"
      }`}>
        <div className="container flex items-center justify-between">
          
          {/* Интегрированный логотип */}
          <div className="flex items-center gap-3 cursor-pointer group" onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })}>
            <div className="relative w-10 h-10 bg-[#001E4E] rounded-md flex items-center justify-center border border-white/20 transition-transform duration-300 group-hover:scale-105">
              <span className="font-display font-extrabold text-lg tracking-wider text-white">EV</span>
              <div className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-4 h-0.5 bg-[#2A9D8F]" />
            </div>
            <div className="flex flex-col">
              <span className="font-display font-bold text-sm tracking-widest text-white">EV INVESTMENT</span>
              <span className="text-[10px] text-gray-400 font-mono tracking-widest">QUY NHON • VIETNAM</span>
            </div>
          </div>

          {/* Минималистичное меню */}
          <nav className="hidden md:flex items-center gap-8">
            <a href="#hero" className="text-sm font-medium text-gray-300 hover:text-white transition-colors">Главная</a>
            <a href="#portfolio" className="text-sm font-medium text-gray-300 hover:text-white transition-colors">Портфель</a>
            <a href="#research" className="text-sm font-medium text-gray-300 hover:text-[#2A9D8F] transition-colors flex items-center gap-1">
              Аналитика <span className="text-[9px] bg-[#2A9D8F]/20 text-[#2A9D8F] px-1.5 py-0.5 rounded font-mono">NEW</span>
            </a>
            <a href="#team" className="text-sm font-medium text-gray-300 hover:text-white transition-colors">Команда</a>
          </nav>

          {/* Кнопка действия в стиле Binance */}
          <div className="flex items-center gap-4">
            <button 
              onClick={() => handlePlaceholderClick("Личный кабинет инвестора")}
              className="hidden sm:inline-flex text-sm font-medium text-gray-300 hover:text-white transition-colors"
            >
              Войти
            </button>
            <Button 
              onClick={() => handlePlaceholderClick("Начать инвестировать")}
              className="bg-[#2A9D8F] hover:bg-[#228074] text-[#060e1e] font-semibold px-5 py-2 rounded transition-all duration-300 active:scale-95 shadow-[0_0_20px_rgba(42,157,143,0.3)]"
            >
              Инвестировать
            </Button>
          </div>

        </div>
      </header>

      {/* 1. HERO PAGE: ИММЕРСИВНЫЙ ПОРТАЛ С ЛОГОТИПОМ (Референс Stronghold Fund) */}
      <section id="hero" className="relative min-h-screen flex items-center pt-24 overflow-hidden">
        
        {/* Кинематографичный фон с заливом Куинён */}
        <div className="absolute inset-0 z-0">
          <img 
            src={HERO_IMAGE} 
            alt="Quy Nhon Bay" 
            className="w-full h-full object-cover object-center opacity-45 scale-105 transition-transform duration-1000"
            style={{ transform: `scale(${1 + (zoomLevel - 1) * 0.05})` }}
          />
          {/* Градиенты для премиального размытия и затемнения */}
          <div className="absolute inset-0 bg-gradient-to-r from-[#060e1e] via-[#060e1e]/80 to-transparent" />
          <div className="absolute inset-0 bg-gradient-to-t from-[#060e1e] via-transparent to-[#060e1e]/50" />
        </div>

        <div className="container relative z-10 grid lg:grid-cols-12 gap-12 items-center">
          
          {/* Левая часть: Предложение и интеграция логотипа */}
          <div className="lg:col-span-7 space-y-8">
            
            <div className="inline-flex items-center gap-2 bg-white/5 border border-white/10 rounded-full px-4 py-1.5 backdrop-blur-md">
              <span className="w-2 h-2 rounded-full bg-[#2A9D8F] animate-pulse" />
              <span className="text-xs font-mono text-gray-300 tracking-wider">СУВЕРЕННЫЙ ИНВЕСТИЦИОННЫЙ КЛАСС</span>
            </div>

            <div className="space-y-4">
              <h1 className="text-4xl md:text-6xl lg:text-7xl font-display font-extrabold tracking-tight leading-[1.1]">
                Инвестиции в <br />
                <span className="text-transparent bg-clip-text bg-gradient-to-r from-[#2A9D8F] via-[#F2C94C] to-[#E58AAE]">
                  Будущее Quy Nhon
                </span>
              </h1>
              <p className="text-lg md:text-xl text-gray-300 max-w-xl font-light leading-relaxed">
                Симбиоз институциональной надежности и передовой аналитики на самом динамично развивающемся побережье Вьетнама.
              </p>
            </div>

            {/* Блок интеграции логотипа с предложением (Stronghold Metaphor) */}
            <div className="p-6 rounded-lg bg-white/5 border border-white/10 backdrop-blur-md max-w-lg space-y-4">
              <div className="flex items-start gap-4">
                {/* Символический "прорез" логотипа в предложение */}
                <div className="p-3 bg-[#001E4E] rounded-md border border-white/20">
                  <span className="font-display font-extrabold text-2xl tracking-wider text-white">EV</span>
                </div>
                <div>
                  <h4 className="font-display font-bold text-white text-base">Интегрированная модель EV</h4>
                  <p className="text-xs text-gray-400 mt-1">
                    Мы объединяем собственный капитал фонда, государственные концессии и искусственный интеллект для точечного выбора девелоперских объектов.
                  </p>
                </div>
              </div>
              <div className="pt-2 border-t border-white/10 flex justify-between items-center text-xs">
                <span className="text-gray-400">Текущий портфель под управлением:</span>
                <span className="font-mono font-bold text-[#F2C94C]">$142.8M USD</span>
              </div>
            </div>

            <div className="flex flex-wrap gap-4 pt-4">
              <Button 
                onClick={() => {
                  document.getElementById('portfolio')?.scrollIntoView({ behavior: 'smooth' });
                }}
                className="bg-[#2A9D8F] hover:bg-[#228074] text-[#060e1e] font-semibold px-8 py-6 rounded text-base transition-all duration-300 shadow-[0_0_30px_rgba(42,157,143,0.2)]"
              >
                Исследовать объекты
                <ChevronRight className="w-5 h-5 ml-2" />
              </Button>
              <Button 
                onClick={() => {
                  document.getElementById('research')?.scrollIntoView({ behavior: 'smooth' });
                }}
                variant="outline" 
                className="border-white/20 hover:bg-white/10 text-white font-medium px-8 py-6 rounded text-base"
              >
                Читать исследования
              </Button>
            </div>

          </div>

          {/* Правая часть: Интерактивный терминал / Дашборд Quy Nhon (Симбиоз Binance / Stronghold) */}
          <div className="lg:col-span-5">
            <div className="rounded-xl border border-white/10 bg-[#0c1626]/80 backdrop-blur-xl shadow-2xl overflow-hidden">
              
              {/* Хедер терминала */}
              <div className="bg-[#080f1b] px-6 py-4 border-b border-white/10 flex justify-between items-center">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 rounded-full bg-[#DA251D]" />
                  <div className="w-3 h-3 rounded-full bg-[#F2C94C]" />
                  <div className="w-3 h-3 rounded-full bg-[#2A9D8F]" />
                  <span className="text-xs text-gray-400 font-mono ml-2">EV_TERMINAL_v4.1</span>
                </div>
                <span className="text-[10px] bg-[#2A9D8F]/20 text-[#2A9D8F] px-2 py-0.5 rounded font-mono uppercase tracking-wider">LIVE DATA</span>
              </div>

              {/* Содержимое дашборда */}
              <div className="p-6 space-y-6">
                
                {/* Выбор вкладок в стиле Binance */}
                <div className="flex border-b border-white/10">
                  <button 
                    onClick={() => setActiveTab("dashboard")}
                    className={`pb-3 text-sm font-medium transition-all relative ${activeTab === "dashboard" ? "text-white" : "text-gray-400 hover:text-white"}`}
                  >
                    Аналитика Quy Nhon
                    {activeTab === "dashboard" && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-[#2A9D8F]" />}
                  </button>
                  <button 
                    onClick={() => setActiveTab("calculator")}
                    className={`ml-6 pb-3 text-sm font-medium transition-all relative ${activeTab === "calculator" ? "text-white" : "text-gray-400 hover:text-white"}`}
                  >
                    Инвест-калькулятор
                    {activeTab === "calculator" && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-[#2A9D8F]" />}
                  </button>
                </div>

                {activeTab === "dashboard" ? (
                  <div className="space-y-4 animate-fadeIn">
                    <div className="grid grid-cols-2 gap-4">
                      <div className="p-4 rounded bg-white/5 border border-white/5">
                        <span className="text-xs text-gray-400 block">Средний рост цен (год)</span>
                        <span className="text-2xl font-display font-bold text-white mt-1 flex items-center gap-1">
                          +16.4%
                          <TrendingUp className="w-4 h-4 text-[#2A9D8F]" />
                        </span>
                      </div>
                      <div className="p-4 rounded bg-white/5 border border-white/5">
                        <span className="text-xs text-gray-400 block">Доходность аренды</span>
                        <span className="text-2xl font-display font-bold text-[#F2C94C] mt-1">
                          8.2% - 11.5%
                        </span>
                      </div>
                    </div>

                    <div className="p-4 rounded bg-white/5 border border-white/5 space-y-3">
                      <div className="flex justify-between text-xs text-gray-400">
                        <span>Заполняемость объектов фонда:</span>
                        <span className="text-white font-mono font-bold">94.8%</span>
                      </div>
                      <div className="w-full bg-white/10 h-1.5 rounded-full overflow-hidden">
                        <div className="bg-[#2A9D8F] h-full rounded-full" style={{ width: '94.8%' }} />
                      </div>
                      <p className="text-[11px] text-gray-400">
                        * Данные верифицированы независимым аудитором SWF Institute на основе Q1 2026.
                      </p>
                    </div>

                    {/* Ссылка на подробный дашборд */}
                    <button 
                      onClick={() => handlePlaceholderClick("Полный терминал аналитики")}
                      className="w-full py-3 rounded bg-white/5 hover:bg-white/10 border border-white/10 text-xs font-mono text-center transition-colors flex items-center justify-center gap-2"
                    >
                      ОТКРЫТЬ ПОЛНЫЙ ИНСТИТУЦИОНАЛЬНЫЙ ДАШБОРД
                      <ArrowUpRight className="w-3.5 h-3.5" />
                    </button>
                  </div>
                ) : (
                  <div className="space-y-4 animate-fadeIn">
                    <div className="space-y-2">
                      <label className="text-xs text-gray-400 block">Сумма инвестиций (USD)</label>
                      <div className="relative">
                        <input 
                          type="text" 
                          defaultValue="250,000" 
                          className="w-full bg-white/5 border border-white/10 rounded px-4 py-3 text-white font-mono focus:outline-none focus:border-[#2A9D8F]"
                        />
                        <span className="absolute right-4 top-1/2 -translate-y-1/2 text-xs font-mono text-gray-400">USD</span>
                      </div>
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                      <div className="p-3 rounded bg-white/5 border border-white/5">
                        <span className="text-[11px] text-gray-400 block">Прогноз прибыли (5 лет)</span>
                        <span className="text-lg font-mono font-bold text-[#2A9D8F] mt-1">+$218,500</span>
                      </div>
                      <div className="p-3 rounded bg-white/5 border border-white/5">
                        <span className="text-[11px] text-gray-400 block">Средний ROI</span>
                        <span className="text-lg font-mono font-bold text-[#F2C94C] mt-1">14.6% / год</span>
                      </div>
                    </div>

                    <Button 
                      onClick={() => handlePlaceholderClick("Расчет финансовой модели")}
                      className="w-full bg-[#2A9D8F] hover:bg-[#228074] text-[#060e1e] font-bold py-3 rounded"
                    >
                      Получить детальный инвест-план
                    </Button>
                  </div>
                )}

              </div>
            </div>
          </div>

        </div>

        {/* Скролл-индикатор */}
        <div className="absolute bottom-8 left-1/2 -translate-x-1/2 z-10 flex flex-col items-center gap-2">
          <span className="text-[10px] font-mono tracking-widest text-gray-400 uppercase">Листайте вниз</span>
          <ChevronDown className="w-4 h-4 text-gray-400 animate-bounce" />
        </div>

      </section>

      {/* 2. ИНТЕРАКТИВНЫЙ ВЫБОР ОБЪЕКТОВ С МЕТАФОРОЙ ПРИБЛИЖЕНИЯ (Референс Stronghold) */}
      <section id="portfolio" className="py-24 border-t border-white/5 relative bg-[#081020]">
        
        <div className="container">
          
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-16">
            <div className="space-y-4">
              <span className="text-xs font-mono text-[#2A9D8F] tracking-widest uppercase">Инвестиционный портфель</span>
              <h2 className="text-3xl md:text-5xl font-display font-bold tracking-tight">
                Интерактивная карта объектов
              </h2>
            </div>
            <p className="text-gray-400 max-w-md mt-4 md:mt-0 text-sm font-light">
              Нажмите на объект на карте или выберите из списка ниже, чтобы запустить метафору приближения (Zoom-in) и изучить подробную финансовую аналитику.
            </p>
          </div>

          <div className="grid lg:grid-cols-12 gap-12 items-start">
            
            {/* Левая колонка: Карта Quy Nhon с точками (Интерактивная метафора) */}
            <div className="lg:col-span-7">
              <div className="relative aspect-[16/10] rounded-xl border border-white/10 bg-[#060e1e] overflow-hidden group shadow-2xl">
                
                {/* Абстрактная стилизованная карта (фон) */}
                <div className="absolute inset-0 bg-[#060e1e] opacity-90">
                  {/* Имитируем очертания побережья с помощью SVG градиентов */}
                  <svg className="w-full h-full opacity-30" viewBox="0 0 800 500">
                    <path d="M150,0 Q250,150 200,250 T300,500 L800,500 L800,0 Z" fill="#001E4E" />
                    <path d="M150,0 Q250,150 200,250 T300,500" fill="none" stroke="#2A9D8F" strokeWidth="2" strokeDasharray="5 5" />
                  </svg>
                </div>

                {/* Основное изображение Quy Nhon, масштабирующееся при выборе объекта */}
                <div className="absolute inset-0 transition-all duration-1000 ease-out" style={{
                  transform: `scale(${selectedObject.id === "obj-1" ? 1.15 : selectedObject.id === "obj-2" ? 1.3 : 1.1}) translate(${selectedObject.id === "obj-1" ? "-5%" : selectedObject.id === "obj-2" ? "10%" : "0%"}, ${selectedObject.id === "obj-1" ? "5%" : selectedObject.id === "obj-2" ? "-5%" : "0%"})`,
                  backgroundImage: `url(${HERO_IMAGE})`,
                  backgroundSize: 'cover',
                  backgroundPosition: 'center',
                  opacity: 0.25
                }} />

                {/* Точки объектов на карте */}
                {INVESTMENT_OBJECTS.map((obj) => (
                  <button
                    key={obj.id}
                    onClick={() => setSelectedObject(obj)}
                    className="absolute z-20 group/pin -translate-x-1/2 -translate-y-1/2 transition-all duration-300"
                    style={{ left: obj.coords.x, top: obj.coords.y }}
                  >
                    <div className={`relative flex items-center justify-center w-8 h-8 rounded-full transition-all duration-300 ${
                      selectedObject.id === obj.id 
                        ? "bg-[#2A9D8F] scale-110 shadow-[0_0_20px_rgba(42,157,143,0.6)]" 
                        : "bg-white/10 hover:bg-white/20 border border-white/20"
                    }`}>
                      <Building className={`w-4 h-4 ${selectedObject.id === obj.id ? "text-[#060e1e]" : "text-white"}`} />
                      
                      {/* Пульсирующий радар вокруг активной точки */}
                      {selectedObject.id === obj.id && (
                        <div className="absolute inset-0 rounded-full border border-[#2A9D8F] animate-ping opacity-75" />
                      )}
                    </div>
                    
                    {/* Тултип при наведении */}
                    <div className="absolute left-1/2 -translate-x-1/2 bottom-10 bg-[#0c1626] border border-white/10 px-3 py-1.5 rounded text-[11px] whitespace-nowrap opacity-0 group-hover/pin:opacity-100 transition-opacity pointer-events-none">
                      <span className="font-bold">{obj.title}</span>
                      <span className="text-[#F2C94C] ml-2">{obj.roi}</span>
                    </div>
                  </button>
                ))}

                {/* Индикатор масштаба */}
                <div className="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded border border-white/10 text-[10px] font-mono text-gray-400">
                  МАСШТАБ КАРТЫ: <span className="text-[#2A9D8F] font-bold">{selectedObject.id === "obj-1" ? "1.15x" : selectedObject.id === "obj-2" ? "1.30x" : "1.10x"}</span>
                </div>

                <div className="absolute top-4 left-4 bg-[#2A9D8F]/10 border border-[#2A9D8F]/20 px-3 py-1 rounded text-[10px] font-mono text-[#2A9D8F] flex items-center gap-1.5">
                  <MapPin className="w-3 h-3" />
                  ПОЛУОСТРОВ QUY NHON, ВЬЕТНАМ
                </div>

              </div>
            </div>

            {/* Правая колонка: Детализация выбранного объекта */}
            <div className="lg:col-span-5 space-y-6">
              
              <div className="p-8 rounded-xl border border-white/10 bg-[#0c1626]/60 backdrop-blur-xl space-y-6">
                
                <div className="space-y-2">
                  <span className="text-xs font-mono text-[#2A9D8F] uppercase tracking-wider">{selectedObject.type}</span>
                  <h3 className="text-2xl font-display font-bold text-white">{selectedObject.title}</h3>
                  <p className="text-xs text-gray-400 flex items-center gap-1">
                    <MapPin className="w-3.5 h-3.5 text-gray-500" />
                    {selectedObject.location}
                  </p>
                </div>

                <p className="text-sm text-gray-300 font-light leading-relaxed">
                  {selectedObject.desc}
                </p>

                <div className="grid grid-cols-3 gap-4 py-6 border-y border-white/10">
                  <div>
                    <span className="text-[10px] text-gray-400 block uppercase font-mono">Доходность</span>
                    <span className="text-lg font-display font-bold text-[#F2C94C] mt-1">{selectedObject.roi}</span>
                  </div>
                  <div>
                    <span className="text-[10px] text-gray-400 block uppercase font-mono">Заполняемость</span>
                    <span className="text-lg font-display font-bold text-white mt-1">{selectedObject.occupancy}</span>
                  </div>
                  <div>
                    <span className="text-[10px] text-gray-400 block uppercase font-mono">Мин. порог</span>
                    <span className="text-lg font-display font-bold text-[#2A9D8F] mt-1">{selectedObject.minInvestment}</span>
                  </div>
                </div>

                {/* Дополнительные преимущества */}
                <div className="space-y-3 text-xs text-gray-300">
                  <div className="flex items-center gap-2">
                    <Shield className="w-4 h-4 text-[#2A9D8F]" />
                    <span>Полное юридическое сопровождение сделки фонда</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Layers className="w-4 h-4 text-[#2A9D8F]" />
                    <span>Возможность долевого участия от 10% стоимости</span>
                  </div>
                </div>

                <div className="pt-2">
                  <Button 
                    onClick={() => handlePlaceholderClick(`Запрос презентации: ${selectedObject.title}`)}
                    className="w-full bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium py-3 rounded text-xs tracking-wider uppercase"
                  >
                    Получить инвест-меморандум объекта
                  </Button>
                </div>

              </div>

              {/* Переключатели объектов в виде списка */}
              <div className="grid grid-cols-3 gap-3">
                {INVESTMENT_OBJECTS.map((obj) => (
                  <button
                    key={obj.id}
                    onClick={() => setSelectedObject(obj)}
                    className={`p-3 rounded-lg border text-left transition-all ${
                      selectedObject.id === obj.id 
                        ? "border-[#2A9D8F] bg-[#2A9D8F]/5" 
                        : "border-white/5 bg-white/5 hover:bg-white/10"
                    }`}
                  >
                    <span className="text-[10px] text-gray-400 block truncate">{obj.type}</span>
                    <span className="text-xs font-bold text-white block truncate mt-1">{obj.title.split(" ")[2] || obj.title}</span>
                  </button>
                ))}
              </div>

            </div>

          </div>

        </div>

      </section>

      {/* 3. ИНСТИТУЦИОНАЛЬНЫЕ ИССЛЕДОВАНИЯ (Референс Bridgewater) */}
      <section id="research" className="py-24 border-t border-white/5 bg-[#060e1e]">
        
        <div className="container">
          
          <div className="max-w-3xl space-y-4 mb-16">
            <span className="text-xs font-mono text-[#2A9D8F] tracking-widest uppercase">Исследования и аналитика</span>
            <h2 className="text-3xl md:text-5xl font-display font-bold tracking-tight">
              EV Research & Insights
            </h2>
            <p className="text-gray-400 text-base font-light leading-relaxed">
              В духе аналитического подхода Bridgewater, мы публикуем регулярные исследования макроэкономики Вьетнама и рынка недвижимости Quy Nhon. Мы верим в прозрачность данных и принятие решений на основе глубокого ресерча.
            </p>
          </div>

          {/* Сетка отчетов в стиле Bridgewater */}
          <div className="grid md:grid-cols-3 gap-8">
            {RESEARCH_REPORTS.map((report) => (
              <div 
                key={report.id}
                className="group p-8 rounded-xl border border-white/10 bg-[#0c1626]/40 hover:bg-[#0c1626]/80 transition-all duration-300 flex flex-col justify-between min-h-[380px]"
              >
                <div className="space-y-6">
                  <div className="flex justify-between items-center text-xs font-mono">
                    <span className="text-gray-400">{report.date}</span>
                    <span className="text-[#2A9D8F] bg-[#2A9D8F]/10 px-2 py-0.5 rounded">{report.category}</span>
                  </div>
                  
                  <h3 className="text-lg font-display font-bold text-white group-hover:text-[#2A9D8F] transition-colors line-clamp-3 leading-snug">
                    {report.title}
                  </h3>
                  
                  <p className="text-xs text-gray-400 line-clamp-4 font-light leading-relaxed">
                    {report.summary}
                  </p>
                </div>

                <div className="pt-8 border-t border-white/5 flex items-center justify-between text-xs">
                  <span className="text-gray-400 font-mono">{report.readTime}</span>
                  <button 
                    onClick={() => handlePlaceholderClick(`Скачивание отчета: ${report.title}`)}
                    className="flex items-center gap-1.5 text-white hover:text-[#2A9D8F] transition-colors font-semibold"
                  >
                    PDF ({report.fileSize})
                    <Download className="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            ))}
          </div>

          {/* Дополнительный блок "Исследования на заказ" для институционалов (SWF Institute style) */}
          <div className="mt-12 p-8 rounded-xl border border-white/10 bg-gradient-to-r from-[#001E4E]/30 to-transparent flex flex-col md:flex-row items-center justify-between gap-6">
            <div className="flex items-center gap-4">
              <div className="p-3 bg-[#2A9D8F]/10 rounded-lg text-[#2A9D8F] hidden sm:block">
                <BookOpen className="w-6 h-6" />
              </div>
              <div>
                <h4 className="font-display font-bold text-white text-base">Индивидуальные исследования для LPs</h4>
                <p className="text-xs text-gray-400 mt-1">
                  Для институциональных партнеров и семейных офисов мы готовим кастомизированные отчеты по запросу.
                </p>
              </div>
            </div>
            <Button 
              onClick={() => handlePlaceholderClick("Запрос кастомного исследования")}
              className="bg-white/5 hover:bg-white/10 border border-white/10 text-white font-medium px-6 py-3 rounded text-xs tracking-wider uppercase whitespace-nowrap"
            >
              Связаться с аналитическим отделом
            </Button>
          </div>

        </div>

      </section>

      {/* 4. СТРАНИЦА TEAM И КОРПОРАТИВНАЯ КУЛЬТУРА (Симбиоз Stronghold / SWF) */}
      <section id="team" className="py-24 border-t border-white/5 bg-[#081020]">
        
        <div className="container">
          
          <div className="grid lg:grid-cols-12 gap-12 items-center mb-16">
            <div className="lg:col-span-6 space-y-4">
              <span className="text-xs font-mono text-[#2A9D8F] tracking-widest uppercase">Управление и экспертиза</span>
              <h2 className="text-3xl md:text-5xl font-display font-bold tracking-tight">
                Наша Команда
              </h2>
              <p className="text-gray-400 text-sm font-light leading-relaxed">
                Команда EV Investment состоит из профессионалов с международным опытом в сфере инвестиций, управления рисками и девелопмента недвижимости. Наша цель — обеспечить максимальную прозрачность и доходность для наших партнеров.
              </p>
            </div>
            <div className="lg:col-span-6">
              {/* Премиальное изображение офиса фонда */}
              <div className="relative aspect-[16/9] rounded-xl overflow-hidden border border-white/10 shadow-2xl">
                <img 
                  src={OFFICE_IMAGE} 
                  alt="EV Investment Office" 
                  className="w-full h-full object-cover opacity-60"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
                <div className="absolute bottom-4 left-4 right-4 flex justify-between items-end">
                  <div className="space-y-1">
                    <span className="text-[10px] font-mono text-[#2A9D8F] uppercase tracking-wider">Главный офис</span>
                    <h4 className="text-sm font-bold text-white">EV Boardroom • Quy Nhon</h4>
                  </div>
                  <span className="text-[10px] text-gray-400 font-mono">Q1 2026</span>
                </div>
              </div>
            </div>
          </div>

          {/* Карточки команды в стиле Stronghold */}
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">
            
            {/* Член команды 1 */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-white/10 bg-[#0c1626]">
                <img 
                  src={TEAM_1} 
                  alt="Minh Nguyen" 
                  className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-6">
                  <p className="text-xs text-gray-300 font-light leading-relaxed">
                    Более 15 лет опыта в управлении суверенными фондами во Вьетнаме и Сингапуре. Экс-директор по инвестициям в VinaCapital.
                  </p>
                </div>
              </div>
              <div>
                <h4 className="font-display font-bold text-white text-base">Минь Нгуен (Minh Nguyen)</h4>
                <p className="text-xs text-[#2A9D8F] font-mono mt-1">Управляющий партнер, сооснователь</p>
              </div>
            </div>

            {/* Член команды 2 */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-white/10 bg-[#0c1626]">
                <img 
                  src={TEAM_2} 
                  alt="Anh Pham" 
                  className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-6">
                  <p className="text-xs text-gray-300 font-light leading-relaxed">
                    Специализируется на оценке рисков и финансовом моделировании прибрежного девелопмента. Ранее работала в CBRE Vietnam.
                  </p>
                </div>
              </div>
              <div>
                <h4 className="font-display font-bold text-white text-base">Ань Фам (Anh Pham)</h4>
                <p className="text-xs text-[#2A9D8F] font-mono mt-1">Директор по исследованиям и рискам</p>
              </div>
            </div>

            {/* Член команды 3 (Заглушка для демонстрации сетки) */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-white/10 bg-[#0c1626] flex items-center justify-center p-6 text-center">
                <div className="space-y-4">
                  <div className="w-12 h-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center mx-auto text-[#2A9D8F]">
                    <Users className="w-6 h-6" />
                  </div>
                  <div>
                    <h5 className="font-display font-bold text-white text-sm">Присоединяйтесь к нам</h5>
                    <p className="text-xs text-gray-400 mt-2 font-light">
                      Мы всегда ищем талантливых аналитиков и управляющих активами в Куинёне.
                    </p>
                  </div>
                  <Button 
                    onClick={() => handlePlaceholderClick("Отправка резюме")}
                    variant="outline" 
                    className="border-white/10 text-xs py-1 h-auto"
                  >
                    Вакансии
                  </Button>
                </div>
              </div>
              <div>
                <h4 className="font-display font-bold text-gray-500 text-base">Открытая позиция</h4>
                <p className="text-xs text-gray-500 font-mono mt-1">Инвестиционный аналитик</p>
              </div>
            </div>

            {/* Член команды 4 (Заглушка для демонстрации сетки) */}
            <div className="group space-y-4">
              <div className="relative aspect-[3/4] rounded-xl overflow-hidden border border-white/10 bg-[#0c1626] flex items-center justify-center p-6 text-center">
                <div className="space-y-4">
                  <div className="w-12 h-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center mx-auto text-[#F2C94C]">
                    <Globe className="w-6 h-6" />
                  </div>
                  <div>
                    <h5 className="font-display font-bold text-white text-sm">Сеть LP партнеров</h5>
                    <p className="text-xs text-gray-400 mt-2 font-light">
                      Более 40 институциональных инвесторов из 12 стран мира доверяют нам свой капитал.
                    </p>
                  </div>
                  <Button 
                    onClick={() => handlePlaceholderClick("Связаться с IR")}
                    variant="outline" 
                    className="border-white/10 text-xs py-1 h-auto"
                  >
                    IR Контакты
                  </Button>
                </div>
              </div>
              <div>
                <h4 className="font-display font-bold text-gray-500 text-base">Инвесторские отношения</h4>
                <p className="text-xs text-gray-500 font-mono mt-1">Investor Relations (IR)</p>
              </div>
            </div>

          </div>

        </div>

      </section>

      {/* 5. ИНСТИТУЦИОНАЛЬНЫЙ ФУТЕР (Симбиоз SWF / Binance) */}
      <footer className="bg-[#040914] border-t border-white/10 py-16">
        <div className="container space-y-12">
          
          <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
            
            {/* О компании */}
            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 bg-[#001E4E] rounded flex items-center justify-center border border-white/20">
                  <span className="font-display font-extrabold text-sm tracking-wider text-white">EV</span>
                </div>
                <span className="font-display font-bold text-sm tracking-widest text-white">EV INVESTMENT</span>
              </div>
              <p className="text-xs text-gray-400 font-light leading-relaxed">
                Специализированный инвестиционный фонд недвижимости, ориентированный на премиальный сегмент побережья города Куинён, провинция Биньдинь, Вьетнам.
              </p>
            </div>

            {/* Навигация */}
            <div className="space-y-4">
              <h5 className="font-display font-bold text-xs tracking-wider uppercase text-white">Навигация</h5>
              <ul className="space-y-2 text-xs text-gray-400">
                <li><a href="#hero" className="hover:text-white transition-colors">Главная</a></li>
                <li><a href="#portfolio" className="hover:text-white transition-colors">Интерактивный портфель</a></li>
                <li><a href="#research" className="hover:text-white transition-colors">Исследования и аналитика</a></li>
                <li><a href="#team" className="hover:text-white transition-colors">Наша команда</a></li>
              </ul>
            </div>

            {/* Документы */}
            <div className="space-y-4">
              <h5 className="font-display font-bold text-xs tracking-wider uppercase text-white">Документы и комплаенс</h5>
              <ul className="space-y-2 text-xs text-gray-400">
                <li><button onClick={() => handlePlaceholderClick("Политика конфиденциальности")} className="hover:text-white transition-colors text-left">Политика конфиденциальности</button></li>
                <li><button onClick={() => handlePlaceholderClick("Условия использования")} className="hover:text-white transition-colors text-left">Условия использования</button></li>
                <li><button onClick={() => handlePlaceholderClick("Дисклеймер рисков")} className="hover:text-white transition-colors text-left">Дисклеймер рисков (Risk Disclosure)</button></li>
                <li><button onClick={() => handlePlaceholderClick("Регистрационные документы")} className="hover:text-white transition-colors text-left">Регистрация в SWF Institute</button></li>
              </ul>
            </div>

            {/* Контакты */}
            <div className="space-y-4">
              <h5 className="font-display font-bold text-xs tracking-wider uppercase text-white">Контакты</h5>
              <ul className="space-y-2 text-xs text-gray-400">
                <li className="flex items-center gap-2">
                  <MapPin className="w-3.5 h-3.5 text-gray-500" />
                  <span>An Phu Thinh Plaza, Quy Nhon, Vietnam</span>
                </li>
                <li className="font-mono">ir@ev-investment.vn</li>
                <li className="font-mono">+84 (256) 388-XXXX</li>
              </ul>
            </div>

          </div>

          {/* Юридический дисклеймер (В стиле SWF / Bridgewater) */}
          <div className="pt-8 border-t border-white/5 text-[10px] text-gray-500 space-y-4 leading-relaxed font-light">
            <p>
              **ВАЖНОЕ ПРЕДУПРЕЖДЕНИЕ О РИСКАХ**: Инвестиции в недвижимость сопряжены с существенными рисками, включая потерю основной суммы инвестиций. Прошлые показатели доходности фонда (ROI YTD) не гарантируют аналогичных результатов в будущем. Информация, представленная на данном сайте, не является индивидуальной инвестиционной рекомендацией или предложением о покупке ценных бумаг. Все решения принимаются инвестором самостоятельно на основе подробного изучения инвестиционного меморандума и консультаций с юридическими и финансовыми советниками.
            </p>
            <div className="flex flex-col sm:flex-row justify-between items-center gap-4 text-gray-400">
              <span>© 2026 EV Investment Fund. Все права защищены.</span>
              <div className="flex items-center gap-4">
                <span className="text-[9px] border border-white/10 px-2 py-0.5 rounded font-mono">SEC COMPLIANT</span>
                <span className="text-[9px] border border-white/10 px-2 py-0.5 rounded font-mono">SWF VERIFIED</span>
              </div>
            </div>
          </div>

        </div>
      </footer>

    </div>
  );
}
