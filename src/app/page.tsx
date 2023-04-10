import { cx } from 'class-variance-authority'
import { Montserrat } from 'next/font/google'

import Greet from '@/components/Greet'

const montserrat = Montserrat({
  subsets: ['latin'],
  variable: '--montserrat',
  weight: ['400', '700'],
})

export default function Home() {
  return (
    <main className={cx(montserrat.variable)}>
      <Greet />
    </main>
  )
}
